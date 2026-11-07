# Create Window Function

## Definition

This function is charged to create the windows from the Window Params.

Signature:
```rust
fn create_window(params: WindowParams, state: &mut App<GuiState>, win_id: WindowId) -> Window;
```

This takes a params, the App, and a window id.

Definitions:
- [WindowParams](../user-guide/WINDOW_PARAMS.md)
- [App\<GuiState\>](../user-guide/APP_STATE.md)
- [WindowId](../user-guide/WINDOW_ID.md)

## Example
Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/c29d3bb97e6352f4c867003809d4af10921ab60e/crates/frontends/ui/src/gui/gui_main.rs#L33), one of my projects)

```rust

fn create_window_safe(
    params: WindowParams,
    app: &mut App<GuiState>,
    win_id: WindowId,
) -> Result<Window, WindowCreationError> {
    let window = match params.wintype {
        WindowType::HomeWindow(first_opening, startup_errors) => {
            let mut win = HomeWindow::new(startup_errors)?;

            win.post_create(app, first_opening)?;

            Window::HomeWindow(win)
        }
        WindowType::ErrorWindow(error) => {
            let mut win = ErrorWindow::new()?;

            win.post_create(error);

            Window::ErrorWindow(win)
        }
        WindowType::SettingsWindow => {
            let mut win = SettingsWindow::new()?;

            win.post_create(app);

            Window::SettingsWindow(win)
        }
        WindowType::StartupWindow => {
            let mut win = StartupWindow::new(win_id)?;

            win.post_create();

            Window::StartupWindow(win)
        }
        WindowType::ResultsWindow(results, query, indexer_id) => {
            let mut win = ResultsWindow::new(indexer_id)?;

            win.post_create(results, query, app);

            Window::ResultsWindow(win)
        }
        WindowType::DetailsWindow(path, indexer_id) => {
            let mut win = DetailsWindow::new()?;

            win.post_create(path, indexer_id, app);

            Window::DetailsWindow(win)
        }
        #[cfg(feature = "ai")]
        WindowType::AiChatWindow(prompt, indexer_id) => {
            let mut win = AiChatWindow::new(indexer_id, app)?;

            win.post_create(prompt, app);

            Window::AiChatWindow(win)
        }
        #[cfg(not(feature = "ai"))]
        #[allow(unused)]
        _ => {
            let mut win = ErrorWindow::new()?;

            win.post_create(format!("Window not supported as standalone Window :("));

            Window::ErrorWindow(win)
        }
    };

    return Ok(window);
}

pub fn text(text: &str) -> XmlElement {
    let mut label = XmlElement::void();

    label.set_tag("Label");
    label.set_text(text);

    return label;
}

fn create_window(params: WindowParams, state: &mut App<GuiState>, win_id: WindowId) -> Window {
    match create_window_safe(params, state, win_id) {
        Ok(win) => win,
        Err(error) => create_window_safe(WindowParams::error(error.to_string()), state, win_id)
            .map_err(|e| e.to_string())
            .expect("Failed to even create the error window: FATAL ERROR"),
    }
}
```

Here I'm using a "safe" version, in case there's an XML error
