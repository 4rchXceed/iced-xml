pub enum WindowType {
    TodoListWindow,
    TodoDetailWindow(usize),
}

pub struct WindowParams {
    pub window: WindowType,
}
