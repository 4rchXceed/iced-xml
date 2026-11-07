use iced_xml::window_manager;

use crate::state::AppState;
use crate::todo_main::TodoListWindow;

mod create_window;
mod params;
mod state;
mod task;
mod todo_main;

window_manager!(Window {
    TodoListWindow,
    TodoDetailWindow
}; AppState; WindowParams);

pub fn main() {}
