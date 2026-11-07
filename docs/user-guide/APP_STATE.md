# The Application State

This is used to share something across the whole app
This is shared in all windows

## The user state
The state is a struct that is shared through the whole app

It doesn't need to have any traits (`Clone`, `Debug`, ...). The only requirement is that it needs to be generated from a function

## The App state
The app state contains some utility functions / datas.
This is auto-generated from a User State.
Take this as a wrapper. The original user state is stored at `.state`

## Example of user state
Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/c29d3bb97e6352f4c867003809d4af10921ab60e/crates/frontends/ui/src/gui/state.rs), one of my projects)
```rust
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::manager::{RunningTask, Task};
use application::indexer::traits::registry::FileIndexerRegistry;
use application::searcher::actions::get::DatabaseGet;
use application::searcher::actions::search::SearchDocument;
#[cfg(feature = "daemon")]
use daemon::manager::DaemonManager;
use indexmap::IndexMap;
use settings::settings::Settings;

pub struct GuiState {
    pub searcher: SearchDocument,
    pub get_db: DatabaseGet,
    pub settings: Arc<Mutex<Settings>>,
    pub registry: Arc<dyn FileIndexerRegistry>,
    pub task_sender: Arc<Sender<Task>>,
    pub tasks: Arc<Mutex<IndexMap<u32, RunningTask>>>,
    #[cfg(feature = "daemon")]
    pub daemon_mgr: Arc<Mutex<DaemonManager>>,
}
```
