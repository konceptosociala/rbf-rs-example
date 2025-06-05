use app::TodoApp;
use clap::Parser;
use relm::Widget;

use crate::{db::DatabaseMode, utils::logger::Logger};

pub mod app;
pub mod db;
pub mod model;
pub mod utils;
pub mod widgets;

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// Database address. This is the URL of the database server.
    /// 
    /// Examples: `127.0.0.1:8000/api`, `https://mywebsite.com/api`
    #[arg(long, short = 'A')]
    addr: String,

    /// Database connection mode. Can be either `http` or `https`.
    #[arg(long, short = 'M')]
    mode: DatabaseMode,
}

fn main() {
    Logger::init();

    TodoApp::run(Args::parse())
        .unwrap_or_else(|e| panic!("Cannot execute todo-app: {e}"));
}
