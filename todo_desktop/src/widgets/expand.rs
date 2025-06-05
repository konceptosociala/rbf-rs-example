use gtk::prelude::*;
use relm::Widget;
use relm_derive::widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpandType {
    Vertical,
    Horizontal,
    Both,
}

#[widget]
impl Widget for Expand {
    fn model(param: ExpandType) -> ExpandType {
        param
    }

    fn update(&mut self, _: ()) {}

    view! {
        gtk::Label {
            label: "",
            hexpand: *self.model == ExpandType::Horizontal || *self.model == ExpandType::Both,
            vexpand: *self.model == ExpandType::Vertical || *self.model == ExpandType::Both,
        },
    }
}