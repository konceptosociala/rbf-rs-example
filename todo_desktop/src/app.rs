use gtk::prelude::*;
use gtk::Inhibit;
use relm::Relm;
use relm::Widget;
use relm_derive::widget;

use crate::event::*;
use crate::utils::*;
use crate::utils::traits::*;
use crate::widgets::header::Header;
use crate::widgets::header::HeaderModel;
use crate::widgets::screens::Screens;

use TodoAppMsg::*;

#[cfg(target_os = "macos")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/macos/gtk.gresource");

#[cfg(target_os = "windows")]
pub const UI_THEME: &[u8] = include_bytes!("../themes/windows/gtk.gresource");

pub struct TodoAppModel {
    pub relm: Relm<TodoApp>,
    pub screens: Screens,
    pub current_screen: usize,
}

#[widget]
impl Widget for TodoApp {
    fn model(relm: &Relm<Self>, _: ()) -> TodoAppModel {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        App::apply_theme();

        TodoAppModel {
            relm: relm.clone(),
            screens: Screens::default(),
            current_screen: 0,
        }
    }

    fn update(&mut self, event: TodoAppMsg) {
        match event {
            SetCurrentScreen(index) => self.model.current_screen = index,
            Quit => TodoApp::quit(),
        }
    }

    view! {
        gtk::Window {
            titlebar: view! {
                Header(HeaderModel {
                    label: "Todo App",
                    win_stream: self.model.relm.stream().clone()
                })
            },
            size: Size(640, 480),
            position: gtk::WindowPosition::Center,
            resizable: false,

            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 10,

                content: match self.model.current_screen {
                    0 => Some(self.model.screens.main.widget()),
                    1 => Some(self.model.screens.error.widget()),
                    _ => None,
                },
            },

            delete_event(_, _) => (Quit, Inhibit(false))
        }
    }
}

impl TodoApp {
    pub fn quit() {
        gtk::main_quit();
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn apply_theme() {
        use gtk::prelude::CssProviderExt as _;

        gio::resources_register(&gio::Resource::from_data(&glib::Bytes::from_static(UI_THEME)).unwrap());

        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/gnome/theme/gtk.css");

        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}