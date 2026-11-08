# Running the app
The method to run the app is: `WindowManager::run_app`

Signature:
```rust
pub fn run_app(
        main_window_params: C,
        gen_state: impl Fn() -> S + 'static,
        engine_settings: EngineSettings,
        create_window: CreateWindowFn<W, C, S>,
        close_mode: CloseMode,
        window_opener: Receiver<C>,
        windows_title: String,
    ) -> AppResult;
```

The method requires a lot of parameters. Here's an explanation:

## `main_window_params`
This will be the `WindowParams` for the window that will open first (the default one if you prefer)

## `gen_state`
The function that will generate your App's state

## `engine_settings`
Just pass `DEFAULT_ENGINE_SETTINGS.clone()`, and don't bother, except if you want other font families, in that case, see [The Font Family Guide](../TODO.md)

## `create_window`
[The create_window function](./CREATE_WINDOW_FN.md)

## `close_mode`
If you want to exit the app once the last window is closed, or wait for an `std::process::exit(0);` or a send in the `window_opener`

## `window_opener`
If you want to open a window from another thread, just send your `WindowParams` to this Receiver.

## `windows_title`
Title of every windows
