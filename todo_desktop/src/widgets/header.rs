#![allow(dead_code)]

use gtk::prelude::*;
use relm::{StreamHandle, Widget};
use relm_derive::{widget, Msg};
use crate::utils::traits::*;
use crate::app;

pub struct Model {
    pub label: &'static str,
    pub win_stream: StreamHandle<app::Msg>
}

#[derive(Msg)]
pub enum Msg {
    
}

#[widget]
impl Widget for Header {
    fn model(param: Model) -> Model {
        param
    }

    fn update(&mut self, event: Msg) {
        match event {
            
        }
    }

    view! {
        gtk::HeaderBar {
            title: Some(self.model.label),
            show_close_button: true,
            start_child: view! {
                gtk::Button {
                    image: view! {
                        gtk::Image {
                            icon_name: Some("list-add-symbolic"),
                            icon_size: gtk::IconSize::Button,
                        }
                    },
                
                    // clicked => HeaderMsg::SetIndex(0),
                },
            },
            
            end_child: view! {
                gtk::Button {
                    image: view! {
                        gtk::Image {
                            icon_name: Some("open-menu-symbolic"),
                            icon_size: gtk::IconSize::Button,
                        }
                    },
                
                    // clicked => HeaderMsg::SetIndex(1),
                },
            },
        }
    }
}