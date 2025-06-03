use gtk::prelude::*;
use gtk::*;
use relm::Relm;
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;
use tokio::runtime::Runtime;

use crate::db;
use crate::db::Database;
use crate::{relm_thread, option_error};
use crate::utils::*;
use crate::utils::traits::*;
use crate::widgets::header;
use crate::widgets::header::Header;
use crate::widgets::screens::ScreenId;
use crate::widgets::screens::Screens;

#[cfg(target_os = "macos")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/macos/gtk.gresource");

#[cfg(target_os = "windows")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/windows/gtk.gresource");

pub struct Model {
    pub relm: Relm<TodoApp>,
    pub screens: Screens,
    pub current_screen: ScreenId,
    pub db: Option<Database>,
}

#[derive(Msg)]
pub enum Msg {
    FetchData,
    DataLoaded(db::Result),
    SetCurrentScreen(ScreenId),
    Quit,
}

use Msg::*;

#[widget]
impl Widget for TodoApp {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        TodoApp::apply_theme();

        relm.stream().emit(FetchData);

        Model {
            relm: relm.clone(),
            screens: Screens::new(relm.clone()),
            current_screen: ScreenId::Main,
            db: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            FetchData => relm_thread! {
                relm: self.model.relm, 
                name: "load data thread",
                msg: DataLoaded,
                result: {
                    log::info!("Fetching data...");
                    
                    let addr = std::env::args()
                        .nth(1)
                        .unwrap_or_else(
                            option_error!("API web address is not specified")
                        );

                    let runtime = Runtime::new().unwrap();

                    runtime.block_on(Database::new(&addr))
                }
            },
            SetCurrentScreen(id) => self.model.current_screen = id,
            DataLoaded(data) => {
                match data {
                    Ok(db) => {
                        self.model.current_screen = ScreenId::Main;
                        self.model.db = Some(db)
                    },
                    Err(e) => {
                        self.model.current_screen = ScreenId::Error;
                        self.show_error(&e.to_string());
                    },
                }
            },
            Quit => TodoApp::quit(),
        }
    }

    view! {
        #[name = "main_window"]
        gtk::Window {
            titlebar: view! {
                Header(header::Model {
                    label: "Todo App",
                    win_stream: self.model.relm.stream().clone()
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
    }
}

impl TodoApp {
    pub fn quit() {
        gtk::main_quit();
    }

    pub fn show_error(&self, msg: &str) {
        let dialog = gtk::MessageDialog::new(
            Some(&self.widgets.main_window),
            DialogFlags::MODAL,
            MessageType::Error,
            ButtonsType::Ok,
            msg,
        );

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.show_all();
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn apply_theme() {
        use gtk::prelude::CssProviderExt as _;
        use gtk::traits::SettingsExt;

        if let Some(settings) = gtk::Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        gio::resources_register(&gio::Resource::from_data(&glib::Bytes::from_static(UI_THEME)).unwrap());

        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/gnome/theme/gtk.css");

        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}