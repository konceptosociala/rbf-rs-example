use error_screen::ErrorScreen;
use main_screen::MainScreen;
use relm::Relm;

use crate::{app::TodoApp, screen_registry};

pub mod main_screen;
pub mod error_screen;

screen_registry! {
    screens: [
        Main => MainScreen(relm.clone()),
        Error => ErrorScreen(relm),
    ],
    args: [
        relm: Relm<TodoApp>,
    ],
}