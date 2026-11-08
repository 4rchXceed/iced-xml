# Dom manipulation

## The query system
To make a query, you need to use the `QueryBuilder` (`self.qb` if you created your window with the Proc Macro)

## VERY IMPORTANT
If you only call `self.qb.some-fn()`, nothing will be executed, you **ALWAYS NEED** to call:
```rust
self.process();
```

**AFTER** declaring a bunch of queries.

Take it like a queue

## The `process` method
This method, as shown abose, will process any pending queries.

Very useful: it will return the last query response

## The elements queries VS generic queries
There are 2 main types of queries:
- element queries (DOM)
- other / generic queries (set_timeout, ...)

## Elements Queries
They are called via `self.qb.b(dom_query);` (`.build_query(dom_query)` for the longer version).

You need to pass a `DomQuery` as parameter. A DomQuery is created from the `Dom` builder. The `Dom` struct contains multiple methods, for example:
- `Dom::get_element_by_id`: by `id=""` attribute
- `Dom::get_elements_by_class` by `id=""` attribute
- ...

You can now run actions, here's one example:
```rust
self.qb.b(Dom::get_element_by_id("to-remove").remove());

self.process();
```
More infos in the [API](../TODO.md) section

## Generic queries
Works the same as the elements queries, but doesn't target a single element and are directly called from `self.qb`

Example:
```rust
self.qb.import_css(include_str!("./style.css").to_string(), false);

self.process();
```

## The `with_callback` and the `then`
There are two callbacks:

### `with_callback`
It's the add event listener callback: `self.qb.b(Dom::xyz::add_event_listener(...)).with_callback(...);`

The `add_event_listener` takes as param an EventListenerType

Callback fn signature:
```rust
fn callback(&mut self, event_response: EventResponse, app: &mut App<AppState>) -> ();
```

Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/c29d3bb97e6352f4c867003809d4af10921ab60e/crates/frontends/ui/src/gui/windows/home.rs#L397), one of my projects):
```rust
self.qb
    .b(Dom::get_element_by_id("search-btn").add_event_listener(EventListenerTypes::Click))
    .with_callback(|this, _, app| {
        this.search(app, SearchType::Normal);
    });

self.process();
```

### `then`
`then` takes a `QueryResponse` as callback.
In this function, you can check if the query was successful.

Example:
```rust
pub fn log_on_error(&mut self, qr: QueryResponse) {
    if !qr.success
        && let Some(e) = qr.error_message
    {
        eprintln!("Error in DOM manipulation: {e}");
    }
}
```

### For more
View the [API documentation](../TODO.md)
