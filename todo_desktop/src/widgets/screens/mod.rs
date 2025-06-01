use error_screen::ErrorScreen;
use main_screen::MainScreen;
use relm::Component;

pub mod main_screen;
pub mod error_screen;
pub struct Screens {
    pub main: Component<MainScreen>,
    pub error: Component<ErrorScreen>,
}

impl Default for Screens {
    fn default() -> Self {
        Screens {
            main: relm::init(()).unwrap(),
            error: relm::init(()).unwrap(),
        }
    }
}