# Making the TodoDetailWindow

## Part 1: setup
Same as the first part, but we need to store the task id

Code (`todo_detail.rs`):
```rust
#[window(AppState)]
pub struct TodoDetailWindow {
    pub task_id: usize,
}
```

We also want a basic new method.
We will just add the task_id as parameter, but the rest will be the same as in the first window:

Code (`todo_detail.rs`):
```rust
impl TodoDetailWindow {
    pub fn new(task_id: usize) -> Self {
        let qb = QueryBuilder::new();
        let engine = XmlEngine::new(
            include_str!("./todo_detail.xml").to_string(),
            DEFAULT_ENGINE_SETTINGS.clone(),
        );

        Self {
            qb,
            engine,
            task_id,
        }
    }
}
```

...and the log_on_error method

Code (`todo_detail.rs`):
```rust
fn log_on_error(&mut self, qr: QueryResponse) {
    if !qr.success
        && let Some(e) = qr.error_message
    {
        eprintln!("Error in DOM manipulation: {e}");
    }
}
```

## Part 2: XML
We just want to show the title and description, that's it!

Code (`todo_detail.xml`):
```xml
<Col>
    <Label id="name">Error loading task</Label>
    <Label id="description">Error loading task</Label>
</Col>
```

## Part 3: Rust
We just want to load the task, and update the text in `name` and `description`

Code (`todo_detail.rs`):
```rust
pub fn init(&mut self, app: &mut App<AppState>) {
    let name = app
        .state
        .tasks
        .get(self.task_id)
        .map(|t| t.name.clone())
        .unwrap_or(String::from("Task not found"));

    let description = app
        .state
        .tasks
        .get(self.task_id)
        .map(|t| t.description.clone())
        .unwrap_or(String::new());

    self.qb
        .b(Dom::get_element_by_id("name").set_property("text", &format!("Name: {name}")))
        .then(Self::log_on_error);
    self.qb
        .b(Dom::get_element_by_id("description")
            .set_property("text", &format!("Description: {description}")))
        .then(Self::log_on_error);

    self.process();
}
```

## Next step
[Running the app](./RUNNING_THE_APP.md)
