pub type Tasks = Vec<Task>;

pub struct Task {
    pub name: String,
    pub description: String,
    pub done: bool,
}
