# Running the app

## `WindowManager::run_app`

See [the user guide](../user-guide/RUN_APP.md)

## Our TODO app
In our case, we just want to write this:

Code (`main.rs`):
```rust
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
```

## Next step
[Some simple CSS](./CSS.md)
