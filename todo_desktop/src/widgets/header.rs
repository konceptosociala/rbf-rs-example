#![allow(dead_code)]

use gtk::prelude::*;
use gtk::*;
use relm::{StreamHandle, Widget};
use relm_derive::{widget, Msg};
use crate::{event::TodoAppMsg, utils::traits::*};

pub struct HeaderModel {
    pub label: &'static str,
    pub win_stream: StreamHandle<TodoAppMsg>
}

#[derive(Msg)]
pub enum HeaderMsg {
    SetIndex(usize)
}

#[widget]
impl Widget for Header {
    fn model(param: HeaderModel) -> HeaderModel {
        param
    }

    fn update(&mut self, event: HeaderMsg) {
        match event {
            HeaderMsg::SetIndex(index) => 
                self.model.win_stream.emit(TodoAppMsg::SetCurrentScreen(index)),
        }
    }

    view! {
        gtk::HeaderBar {
            title: Some(self.model.label),
            show_close_button: true,
            start_child: view! {
                gtk::Button {
                    image: Some(&Image::from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button)),
                
                    clicked => HeaderMsg::SetIndex(0),
                },
            },
            
            end_child: view! {
                gtk::Button {
                    image: Some(&Image::from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Button)),
                
                    clicked => HeaderMsg::SetIndex(1),
                },
            },
        }
    }
}