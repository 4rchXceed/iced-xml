# Window Setup - TodoListWindow

## Note (important)
To avoid redundant information, please read [The Window Base guide](../user-guide/WINDOW_BASE.md)
(You only need to read the "Proc Macro" section)

## The struct
We can use the *brand new* `#[window(...)]` macro!

_Note: the Todos are stored in the State directly_

This will be simple, since we don't have anything to store, just write:

Code (`todo_main.rs`)
```rust
use crate::state::AppState;
use iced_xml_macros::window;

#[window(AppState)]
pub struct TodoListWindow {}
```

## The `new()` method
We want a method that doesn't take any parameter.

_For simplicity we will not do a proper error management, and we'll just use the normal `new()`, and not `try_new()` methods_

We still need to create the XmlEngine and QueryBuilder, (qr and engine, added by the `window` macro)

Code (`todo_main.rs`)
```rust
impl TodoListWindow {
    pub fn new() -> Self {
        let qb = QueryBuilder::new();
        let engine = XmlEngine::new(
            include_str!("./todo_main.xml").to_string(),
            DEFAULT_ENGINE_SETTINGS.clone(),
        );

        Self { qb, engine }
    }
}
```

## The `init()` method
We want another method, that will be ran after the Window's creation, to register all events, and add the datas.

Because this is the Setup part, we won't write anything in it

Code (`todo_main.rs`)
```rust
pub fn init(&mut self, app: &mut App<AppState>) {}
```

## Some utils methods
These methods are optional, but I like to write them anyway

### The Error logger
This one will log when there's an error, with an AddEventListener for example

Code (`todo_main.rs`)
```rust
fn log_on_error(&mut self, qr: QueryResponse) {
    if !qr.success
        && let Some(e) = qr.error_message
    {
        eprintln!("Error in DOM manipulation: {e}");
    }
}
```

## Next steps
Next, we will see some [basic XML Syntax](./1ST_WINDOW_XML.md)
