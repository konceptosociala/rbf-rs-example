use relm_derive::widget;
use relm::Widget;
use gtk::prelude::*;

#[widget]
impl Widget for MainScreen {
    fn model() {}

    fn update(&mut self, _: ()) {}

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            spacing: 10,

            gtk::Label {
                label: "Main Screen",
                halign: gtk::Align::Center,
            },
        }
    }
}