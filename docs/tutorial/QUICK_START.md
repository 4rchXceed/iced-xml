# Quick Start

In this tutorial we will build a **TODO App**, with a detail view, and possiblity to Add, and mark TODOs as done.

For simplicity, we won't implement the css-watcher

## Example structure
A base iced-xml structure looks like this:
```
.
├── Cargo.toml
├── src
│   ├── main.rs: main UI setup (explained in user-guide section)
│   ├── window_main.rs: 1st window (explained in user-guide, api and elements sections)
│   └── window_main.xml: 1st window XML structure (explained in elements section)
└── resources
    └── styles
        └── main.css | window_main.css: the style (explained in css section)
```

You can also separate main.rs into `state.rs`, `params.rs` and `create_window.rs`

## Our structure
```
.
├── Cargo.toml
├── src
│   ├── main.rs: Main
│   ├── state.rs: The State
│   ├── params.rs: Window params
│   ├── create_window.rs: create_window fn
│   ├── task.rs: A Task
│   ├── todo_main.rs: Main TODO list
│   ├── todo_main.xml
│   ├── todo_detail.rs: Detail TODO list
│   └── todo_detail.xml
└── resources
    └── styles
        └── main.css | window_main.css: the style
```

## Next: [Building the main UI setup (`main.rs`)](./UI_SETUP.md)
