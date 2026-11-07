# Window Base
For a struct to be a Window, it needs to have a bunch of methods (mainly to implement WindowTemplate<...>).

Here a detailed explaination on _how_ to create a window:

All examples are from: [FFSearch.rs ErrorWindow](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/branch/master/crates/frontends/ui/src/gui/windows/winerror.rs)

## Proc Macro
Because writing all of that is boring, I did a proc macro to do the job for you!

Now you can simply type:
```rust
#[window(AppState)]
pub struct WindowMain {...}
```

That's it!

## Note on CSS-Watcher
Every lines/methods with `#[cfg(debug_assertions)]` before can be ignored if you don't plan to include hot reload in your app

## Minimal fields
Here's an example of minimal fields for a window

```rust
pub struct ErrorWindow {
    qb: QueryBuilder<Self, GuiState>,
    engine: XmlEngine,
    #[cfg(debug_assertions)]
    css_rx: Option<CssRx>,
}
```

- `qb`: this is the query builder. This is an utility class, and doesn't need any context. It's recommended to put it in the class fields so you can just do `self|this.qb`. And the types props?
  - `Self`: The window struct
  - `GuiState`: The [state](./APP_STATE.md)
- `engine`: This is the core of the app. You will never call it directly, the query builder will, but you NEED to save it. TODOC: Add arch/engine.md page
- `css_rx`: this is a property that the css watcher needs to save (optional, required if feature css-watcher enabled)

## Methods
You then need to implement `WindowTemplate<Self, GuiState>`
- `Self`: The window struct
- `GuiState`: The [state](./APP_STATE.md)

```rust
impl WindowTemplate<Self, GuiState> for ErrorWindow {...}
```

### get_objects
This returns a list of mutable references of the Engine, Query Builder, etc.

Example:
```rust
fn get_objects(&mut self) -> Objects<'_, Self, GuiState> {
    return Objects {
        engine: &mut self.engine,
        qb: &mut self.qb,
        #[cfg(debug_assertions)]
        css_path: "styles/mainstyle.css",
        #[cfg(debug_assertions)]
        css_watcher_rx: self.css_rx.as_mut(),
    };
}
```
- `css_path`: Path to the hot-reloaded `css` file

### get_objects_read_only
Same as `get_objects`, but for read-only references, and no requirement for css-watcher thingies

Example:
```rust
fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, Self, GuiState> {
    return ObjectsReadOnly {
        engine: &self.engine,
        qb: &self.qb,
    };
}
```

### get_self
Returns self.

```rust
fn get_self(&mut self) -> &mut Self {
    return self;
}
```

### into_any
Returns Any.

```rust
fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
    return self;
}
```
### (css-watcher) set_css_watcher_rx
Sets the "css-watcher-rx"

Example:
```rust
#[cfg(debug_assertions)]
fn set_css_watcher_rx(&mut self, rx: CssRx) {
    self.css_rx = Some(rx);
}
```

### (Optional) on_close
Action to run when the window closes

Example:
```rust
fn on_close(&mut self, app_state: &mut App<AppState>) {
    println!("Window closed!");
}
```

## NEVER TOUCH:
`update, add_component, remove_component, process, render, process_context, subscription`

## Optional, but recommended methods
Self-defined `post_create` method
You can register any events / update the dom here

Example:
```rust
pub fn post_create(&mut self, error: String) {
    self.qb
        .b(Dom::get_element_by_id("error-msg").set_property("text", error.as_str()));

    self.qb.import_css(crate::gui::css::CSS.to_string(), false);

    self.process();
}
```
