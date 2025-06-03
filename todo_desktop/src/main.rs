use app::TodoApp;
use relm::Widget;

use crate::utils::logger::Logger;

pub mod app;
pub mod db;
pub mod model;
pub mod utils;
pub mod widgets;

fn main() {
    Logger::init();

    TodoApp::run(())
        .unwrap_or_else(|e| panic!("Cannot execute todo-app: {e}"));
}
