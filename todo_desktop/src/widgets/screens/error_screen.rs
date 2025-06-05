use relm_derive::{widget, Msg};
use relm::{Relm, StreamHandle, Widget};
use gtk::prelude::*;
use gtk::*;

use crate::app::{self, TodoApp};

pub struct Model {
    pub app_stream: StreamHandle<app::Msg>
}

#[derive(Msg)]
pub enum Msg {
    RetryFetch,
}

use Msg::*;

#[widget]
impl Widget for ErrorScreen {
    fn model(relm: Relm<TodoApp>) -> Model {
        Model {
            app_stream: relm.stream().clone()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            RetryFetch => self.model.app_stream.emit(app::Msg::GetTasks),
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Vertical,
            spacing: 10,
            halign: Align::Center,
            valign: Align::Center,
            hexpand: true,
            vexpand: true,

            gtk::Image {
                icon_name: Some("dialog-information-symbolic"),
                icon_size: gtk::IconSize::Dialog,
            },

            gtk::Label {
                label: "Cannot connect to the server",
            },

            gtk::Button {
                label: "Try again",
                
                clicked => Msg::RetryFetch,
            }
        }
    }
}