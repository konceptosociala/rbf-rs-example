#![allow(dead_code)]

use relm_derive::{widget, Msg};
use relm::{StreamHandle, Widget};
use gtk::prelude::*;
use gtk::*;

use crate::{
    app, 
    model::task::Task, 
    widgets::expand::*,
};

pub struct Model {
    pub task: Task,
    pub app_stream: StreamHandle<app::Msg>,
}

#[derive(Msg)]
pub enum Msg {
    ToggleTask,
    DeleteTask,
}

use Msg::*;

#[widget]
impl Widget for TaskPanel {
    fn model(param: Model) -> Model {
        param
    }

    fn update(&mut self, event: Msg) {
        match event {
            ToggleTask => {
                self.model.task.completed = !self.model.task.completed;
                self.model.app_stream.emit(app::Msg::UpdateTask(self.model.task.clone()));
            }
            DeleteTask => {
                self.model.app_stream.emit(app::Msg::DeleteTask(self.model.task.clone()));
            }
        }
    }

    view! {
        gtk::Frame {
            gtk::Box {
                orientation: Orientation::Horizontal,
                spacing: 10,
                margin: 10,

                gtk::CheckButton {
                    active: self.model.task.completed,

                    toggled(checked) => ToggleTask,
                },

                gtk::Label {
                    label: &self.model.task.title,
                },

                Expand(ExpandType::Horizontal),

                gtk::Button {
                    image: Some(&Image::from_icon_name(
                        Some("user-trash-symbolic"), 
                        gtk::IconSize::Button,
                    )),

                    clicked => DeleteTask,
                },
            }
        }
    }
}