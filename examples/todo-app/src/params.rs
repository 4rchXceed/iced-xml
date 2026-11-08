#[derive(Clone)]
pub enum WindowType {
    TodoListWindow,
    TodoDetailWindow(usize),
}

#[derive(Clone)]
pub struct WindowParams {
    pub window: WindowType,
}
