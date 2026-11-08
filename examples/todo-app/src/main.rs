use iced_xml::app_manager::app_manager::WindowManager;
use iced_xml::app_manager::utils::CloseMode;
use iced_xml::window_manager;
use iced_xml::window_wrapper::AppResult;

use crate::create_window::create_window;
use crate::params::{WindowParams, WindowType};
use crate::state::AppState;
use crate::todo_detail::TodoDetailWindow;
use crate::todo_main::TodoListWindow;

mod create_window;
mod params;
mod state;
mod task;
mod todo_detail;
mod todo_main;

window_manager!(Window {
    TodoListWindow,
    TodoDetailWindow
}; AppState; WindowParams);

pub fn main() -> AppResult {
    return WindowManager::run_app(
        WindowParams {
            window: WindowType::TodoListWindow,
        },
        || AppState { tasks: Vec::new() },
        DEFAULT_ENGINE_SETTINGS.clone(),
        create_window,
        CloseMode::Close,
        std::sync::mpsc::channel::<WindowParams>().1,
        String::from("My TODO App"),
    );
}
