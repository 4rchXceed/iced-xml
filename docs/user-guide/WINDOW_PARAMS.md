# WindowParams
The window params are a set of parameters that will be passed to your code when a window wants to create another window, you will then process these datas so you can create the correct window in the correct state

It needs to be:
- `#[derive(Clone)]`


Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/c29d3bb97e6352f4c867003809d4af10921ab60e/crates/frontends/ui/src/gui/params.rs#L4), one of my projects)
```rust
#[derive(Clone)]
pub enum WindowType {
    HomeWindow(bool, Vec<String>),
    ErrorWindow(String),
    SettingsWindow,
    StartupWindow,
    ResultsWindow(Vec<Document>, String, IndexerId),
    DetailsWindow(String, IndexerId),
    #[cfg(feature = "ai")]
    AiChatWindow(String, IndexerId),
}

#[derive(Clone)]
pub struct WindowParams {
    pub wintype: WindowType,
}
```
