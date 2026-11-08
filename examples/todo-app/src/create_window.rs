use iced_xml::app_manager::{state_wrapper::App, utils::WindowId};

use crate::{
    Window,
    params::{
        WindowParams,
        WindowType::{self},
    },
    state::AppState,
    todo_detail::TodoDetailWindow,
    todo_main::TodoListWindow,
};

pub fn create_window(params: WindowParams, state: &mut App<AppState>, _win_id: WindowId) -> Window {
    match params.window {
        WindowType::TodoListWindow => {
            let mut win = TodoListWindow::new();

            win.init(state);

            Window::TodoListWindow(win)
        }
        WindowType::TodoDetailWindow(task_id) => {
            let mut win = TodoDetailWindow::new(task_id);

            win.init(state);

            Window::TodoDetailWindow(win)
        }
    }
}
