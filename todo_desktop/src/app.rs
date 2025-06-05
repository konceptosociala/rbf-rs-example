#![allow(dead_code)]

use std::sync::Arc;
use gtk::prelude::*;
use gtk::*;
use relm::Relm;
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;
use tokio::runtime::Runtime;

use crate::db;
use crate::db::Database;
use crate::db::DatabaseContext;
use crate::model::task::Task;
use crate::widgets::screens::main_screen;
use crate::widgets::screens::Screen;
use crate::Args;
use crate::relm_thread;
use crate::utils::*;
use crate::utils::traits::*;
use crate::widgets::header;
use crate::widgets::header::Header;
use crate::widgets::screens::ScreenId;
use crate::widgets::screens::Screens;

pub struct Model {
    pub relm: Relm<TodoApp>,
    pub runtime: Arc<Runtime>,
    pub ctx: DatabaseContext,
    pub screens: Screens,
    pub current_screen: ScreenId,
    pub db: Option<Database>,
}

#[derive(Msg)]
pub enum Msg {
    GetTasks,
    AddTask,
    UpdateTask(Task),
    DeleteTask(Task),

    GetTasksResult(db::Result<Database>),
    AddTaskResult(db::Result<Task>),
    UpdateTaskResult(db::Result<Task>),
    DeleteTaskResult(db::Result<Task>),

    OpenAddTaskDialog,
    CancelAddTask,
    SetCurrentScreen(ScreenId),
    UpdateTaskPanels(Vec<Task>),
    Quit,
}

use Msg::*;

#[widget]
impl Widget for TodoApp {
    fn model(relm: &Relm<Self>, args: Args) -> Model {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        TodoApp::apply_theme();

        relm.stream().emit(GetTasks);

        Model {
            runtime: Arc::new(Runtime::new().unwrap()),
            relm: relm.clone(),
            ctx: DatabaseContext::new(&args.addr, args.mode),
            screens: Screens::new(relm.clone()),
            current_screen: ScreenId::Main,
            db: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            // Requests
            GetTasks => {
                let runtime = self.model.runtime.clone();
                let ctx = self.model.ctx.clone();

                relm_thread! {
                    relm: self.model.relm, 
                    name: "get tasks thread",
                    msg: GetTasksResult,
                    result: {                   
                        runtime.block_on(ctx.get_tasks())
                    }
                }
            },
            AddTask => {
                let runtime = self.model.runtime.clone();
                let ctx = self.model.ctx.clone();
                let title = self.widgets.add_task_entry.text();

                if title.is_empty() {
                    self.show_error(anyhow::anyhow!("Task title cannot be empty"), Some(1));
                    return;
                }

                relm_thread! {
                    relm: self.model.relm, 
                    name: "add task thread",
                    msg: AddTaskResult,
                    result: {                   
                        runtime.block_on(ctx.add_task(Task::new(title)))
                    }
                }

                self.widgets.add_task_dialog.hide();
            },
            UpdateTask(task) => {
                let runtime = self.model.runtime.clone();
                let ctx = self.model.ctx.clone();

                relm_thread! {
                    relm: self.model.relm, 
                    name: "update task thread",
                    msg: UpdateTaskResult,
                    result: {                   
                        runtime.block_on(ctx.update_task(task))
                    }
                }
            },
            DeleteTask(task) => {
                let runtime = self.model.runtime.clone();
                let ctx = self.model.ctx.clone();

                relm_thread! {
                    relm: self.model.relm, 
                    name: "delete task thread",
                    msg: DeleteTaskResult,
                    result: {                   
                        runtime.block_on(ctx.delete_task(task))
                    }
                }
            },

            // Responses
            GetTasksResult(result) => {
                match result {
                    Ok(db) => {
                        self.model.current_screen = ScreenId::Main;

                        self.model.relm.stream().emit(UpdateTaskPanels(db.tasks.clone()));
                        self.model.db = Some(db);
                    },
                    Err(e) => {
                        self.model.current_screen = ScreenId::Error;
                        self.show_error(e.into(), Some(2));
                    },
                }
            },
            AddTaskResult(result) => {
                match result {
                    Ok(task) => {
                        if let Some(db) = &mut self.model.db {
                            db.tasks.push(task);

                            self.model.relm.stream().emit(UpdateTaskPanels(db.tasks.clone()));
                        }
                    },
                    Err(e) => {
                        self.show_error(e.into(), Some(2));
                    },
                }
            },
            UpdateTaskResult(result) => {
                match result {
                    Ok(task) => {
                        if let Some(db) = &mut self.model.db {
                            if let Some(t) = db.tasks
                                .iter_mut()
                                .find(|t| t.id == task.id)
                            {
                                *t = task;
                            }

                            self.model.relm.stream().emit(UpdateTaskPanels(db.tasks.clone()));
                        }
                    },
                    Err(e) => {
                        self.show_error(e.into(), Some(2));
                    },
                }
            },
            DeleteTaskResult(result) => {
                match result {
                    Ok(task) => {
                        if let Some(db) = &mut self.model.db {
                            if let Some(idx) = db.tasks
                                .iter()
                                .position(|t| t.id == task.id)
                            {
                                db.tasks.remove(idx);
                            }

                            self.model.relm.stream().emit(UpdateTaskPanels(db.tasks.clone()));
                        }
                    },
                    Err(e) => {
                        self.show_error(e.into(), Some(2));
                    },
                }
            },

            // App control
            OpenAddTaskDialog => {
                if self.model.current_screen != ScreenId::Main {
                    return;
                }

                self.widgets.add_task_entry.set_text("");
                self.widgets.add_task_dialog.set_transient_for(Some(&self.widgets.main_window));
                self.widgets.add_task_dialog.set_position(WindowPosition::CenterOnParent);
                self.widgets.add_task_dialog.present();
            },
            CancelAddTask => {
                self.widgets.add_task_dialog.hide();
                self.widgets.add_task_entry.set_text("");
            },
            UpdateTaskPanels(tasks) => {
                if let Screen::Main(screen) = &self.model.screens[ScreenId::Main] {
                    screen.emit(main_screen::Msg::SetTasks(tasks));
                }
            },
            SetCurrentScreen(id) => self.model.current_screen = id,
            Quit => TodoApp::quit(),
        }
    }

    view! {
        #[name = "main_window"]
        gtk::Window {
            titlebar: view! {
                Header(header::Model {
                    label: "Todo App",
                    app_stream: self.model.relm.stream().clone()
                })
            },
            size: Size(640, 480),
            position: WindowPosition::Center,
            resizable: false,

            gtk::Box {
                orientation: Orientation::Vertical,
                spacing: 10,

                content: Some(
                    self.model.screens[self.model.current_screen].widget()
                ),
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }

        #[name = "add_task_dialog"]
        gtk::Dialog {
            title: "Add Task",
            modal: true,
            type_hint: gdk::WindowTypeHint::Splashscreen,
            content: view! {
                gtk::Box {
                    orientation: Orientation::Vertical,
                    spacing: 10,
                    margin: 10,

                    gtk::Label {
                        label: "Task name"
                    },

                    #[name = "add_task_entry"]
                    gtk::Entry {
                        placeholder_text: Some("Enter new task name")
                    },

                    gtk::Button {
                        label: "OK",

                        clicked => AddTask,
                    },

                    gtk::Button {
                        label: "Cancel",

                        clicked => CancelAddTask,
                    },
                }
            },

            delete_event(_, _) => (CancelAddTask, Inhibit(true)),
        }
    }
}

impl TodoApp {
    pub fn quit() {
        gtk::main_quit();
    }

    pub fn show_error(&self, err: anyhow::Error, depth: Option<usize>) {
        let mut msg = format!("Error: {err}\n");
        for cause in err.chain().skip(1).take(depth.unwrap_or(usize::MAX)) {
            msg.push_str(&format!("Caused by: {cause}\n"));
        }

        let dialog = gtk::MessageDialog::new(
            Some(&self.widgets.main_window),
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Ok,
            &msg,
        );

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.show_all();
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn apply_theme() {
        use crate::*;
        use gtk::traits::SettingsExt;

        if let Some(settings) = gtk::Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        #[cfg(target_os = "macos")]
        include_gresource!("../themes/macos/gtk.gresource");

        #[cfg(target_os = "windows")]
        include_gresource!("../themes/windows/gtk.gresource");

        apply_css!(resource: "/org/gnome/theme/gtk.css");

        #[cfg(target_os = "macos")]
        apply_css!(data: b"
            .task-panel {
                padding: 3px;
                border-radius: 5px;
                background-color: shade(@theme_bg_color, 0.8);
            }

            .delete-button {
                border-radius: 50%;
                padding: 5px;
            }
        ");
    }
}