use app::TodoApp;
use relm::Widget;

pub mod app;
pub mod event;
pub mod model;
pub mod utils;
pub mod widgets;

fn main() {
    TodoApp::run(())
        .unwrap_or_else(|e| panic!("Cannot execute todo-app: {e}"));
}
