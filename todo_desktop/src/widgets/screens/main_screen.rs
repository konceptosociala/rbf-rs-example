use relm_derive::{widget, Msg};
use relm::{Component, Relm, StreamHandle, Widget};
use gtk::prelude::*;
use gtk::*;

use crate::app::{self, TodoApp};
use crate::model::task::Task;
use crate::widgets::task::{self, TaskPanel};

pub struct Model {
    pub app_stream: StreamHandle<app::Msg>,
    pub panels: Vec<Component<TaskPanel>>,
}

#[derive(Msg)]
pub enum Msg {
    SetTasks(Vec<Task>),
}

#[widget]
impl Widget for MainScreen {
    fn model(param: Relm<TodoApp>) -> Model {
        Model {
            app_stream: param.stream().clone(),
            panels: vec![]
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SetTasks(tasks) => {
                for child in self.root().children() {
                    self.root().remove(&child);
                }

                self.model.panels.clear();

                for task in tasks {
                    let panel = relm::init(
                        task::Model {
                            task,
                            app_stream: self.model.app_stream.clone(),
                        }
                    ).unwrap();

                    self.model.panels.push(panel);
                }

                for panel in &self.model.panels {
                    self.root().add(panel.widget());
                }
            }
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Vertical,
            margin: 10,
            spacing: 10,
        }
    }
}