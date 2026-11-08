# Rust queries - TodoListWindow

## Please read the [Dom Manipulation Page](../user-guide/DOM_MANIPULATION.md) first
## And also the [XML Element](../user-guide/XML_ELEMENT.md) user guide

## The `reload_tasks` method
We want to create a method that will update the Tasks shown.

To do that, we will create a (`<Col>`) XmlElement containing every tasks, and then replace it.

The "View more" and "Done" buttons will have the task id as `data-taskid` attribute

Our code will look like this:

Code (`todo_main.rs` -> inside the impl):
```rust
fn reload_tasks(&mut self, app: &mut App<AppState>) {
    let mut tasks_container = XmlElement::void();
    tasks_container.set_tag("Col");
    tasks_container.set_id("tasks-list");

    for (id, task) in app.state.tasks.iter().enumerate() {
        let mut task_element = XmlElement::void();
        task_element.set_tag("Row");

        let mut task_title = XmlElement::void();
        task_title.set_tag("Label");
        task_title.set_text(&format!(
            "{}: {}",
            task.name,
            if task.done { "Done" } else { "To do" }
        ));
        let mut task_details_btn = xml(
            "<Button classes=\"view-more-btn\" style:border-width=\"1\" style:border-color=\"black\">View more</Button>",
        );
        task_details_btn.add_data("taskid", &id.to_string());

        let mut task_done_btn = xml(
            "<Button classes=\"done-btn\" style:border-width=\"1\" style:border-color=\"black\">Done</Button>",
        );
        task_done_btn.add_data("taskid", &id.to_string());

        task_element.append_child(task_title);
        task_element.append_child(task_details_btn);
        task_element.append_child(task_done_btn);

        tasks_container.append_child(task_element);
    }

    self.qb
        .b(Dom::get_element_by_id("tasks-list").replace(Box::new(tasks_container)))
        .then(Self::log_on_error);


    self.qb
        .b(
            Dom::get_elements_by_class("done-btn")
                .add_event_listener(EventListenerTypes::Click),
        )
        .with_callback(Self::on_done_click)
        .then(Self::log_on_error);

    self.qb
        .b(Dom::get_elements_by_class("view-more-btn")
            .add_event_listener(EventListenerTypes::Click))
        .with_callback(Self::on_view_more_click)
        .then(Self::log_on_error);

    self.process();
}
```

Here I'm using `xml()`, but this function will panic if the XML is invalid.

Also important: CSS styles are applied only for the current elements in the page, because we are re-creating elements, we need to explicitly set the styles.

Small explanation of the styles used:
- border-width: same as in CSS
- border-color: same as in CSS

And because the event listeners works like CSS, we also need to re-create the event listeners inside the reload method

## The `add_task` method
This method will get the values of the elements and add a task

We need to get the content of two elements:
- `#new-todo-name`
- `#new-todo-description`

So our method will look like this:

Code (`todo_main.rs` -> inside the impl):
```rust
fn add_task(&mut self, app: &mut App<AppState>) {
    self.qb
        .b(Dom::get_element_by_id("new-todo-name").get_property("value")).then(Self::log_on_error);

    let name_result = self.process();

    self.qb
        .b(Dom::get_element_by_id("new-todo-description").get_property("value")).then(Self::log_on_error);

    let description_result = self.process();

    if let Some(name) = name_result.data_str
        && let Some(description) = description_result.data_str
    {
        if !name.is_empty() && !description.is_empty() {
            app.state.tasks.push(Task {
                done: false,
                name: name,
                description: description,
            });

            self.reload_tasks(app);
        }
    }
}
```

Here we use the `get_property("value")` to get the value

## The `on_done_click` method
We want to use the EventResponse `target` to get the `data-taskid` data, and then update the state

Code (`todo_main.rs` -> inside the impl):
```rust
fn on_done_click(&mut self, er: EventResponse, app: &mut App<AppState>) {
    if let Some(selector) = er.target {
        self.qb.b(Dom::from(selector).get_data("taskid"));

        let result = self.process();

        if let Some(id_str) = result.data_str
            && let Ok(id) = id_str.parse::<usize>()
        {
            if let Some(task) = app.state.tasks.get_mut(id) {
                task.done = true;

                self.reload_tasks(app);
            }
        }
    }
}
```

## The `on_view_more_click` method
Same as above, but we want to use `self.qb.open_window(...)`

```rust
fn on_view_more_click(&mut self, er: EventResponse, app: &mut App<AppState>) {
    if let Some(selector) = er.target {
        self.qb.b(Dom::from(selector).get_data("taskid"));

        let result = self.process();

        if let Some(id_str) = result.data_str
            && let Ok(id) = id_str.parse::<usize>()
        {
            self.qb.open_window(
                Box::new(WindowParams {
                    window: WindowType::TodoDetailWindow(id),
                }),
                app,
            );

            self.process();
        }
    }
}
```

## Filling the `init` method
We want to:
- register an Event Listener on the "Add" button
- reload a first time

Code (`todo_main.rs` -> inside the impl, replacing the old `init()`):
```rust
pub fn init(&mut self, app: &mut App<AppState>) {
    self.qb
        .b(
            Dom::get_element_by_id("new-todo-btn")
                .add_event_listener(EventListenerTypes::Click),
        )
        .with_callback(|this, _, app| {
            this.add_task(app);
        })
        .then(Self::log_on_error);

    self.reload_tasks(app);
}
```

## Next step
[The detail window](./2ND_WINDOW.md)
