use log::error;
use notify::RecommendedWatcher;
use notify::{Event, Result, Watcher};
use std::{path::Path, sync::mpsc};

use crate::window_wrapper::WindowTemplate;

/// PLEASE DO NOT DROP THE VALUE OF THE CSS FILE!!! ELSE THE WATCHER IS GOING TO DIE!
pub fn watch_css_file<T: WindowTemplate<T, AppState> + 'static, AppState: 'static>(
    me: &mut T,
    interval: i32,
) -> Result<RecommendedWatcher> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    me.set_css_watcher_rx(rx);

    let objects = me.get_objects();

    objects
        .qb
        .import_css(std::fs::read_to_string(&objects.css_path).unwrap(), true)
        .then(|_, event| {
            if !event.success {
                error!(
                    "Error while loading CSS: {}",
                    event.error_message.unwrap_or(String::from("Unknown error"))
                );
            }
        });

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(
        Path::new(objects.css_path),
        notify::RecursiveMode::NonRecursive,
    )?;

    objects.qb.set_interval(interval).with_callback(|me, _, _| {
        if me.get_objects().css_watcher_rx.is_none() {
            error!("css_watcher_rx is None. Make sure to correctly implement set_css_watcher_rx in your AppTemplate implementation.");
            return;
        }

        let ev = me.get_objects().css_watcher_rx.unwrap().try_recv();

        if let Ok(ev) = ev {
            if let Ok(ev) = ev && ev.kind.is_modify() {
                let objects = me.get_objects();

                let css_content = match std::fs::read_to_string(&objects.css_path) {
                    Ok(content) => content,
                    Err(e) => {
                        error!("Error reading CSS file: {}", e);
                        return;
                    }
                };
                objects
                .qb
                .import_css(css_content, true)
                .then(|_, event| {
                    if !event.success {
                        error!("Error while loading CSS: {}", event.error_message.unwrap_or(String::from("Unknown error")));
                    }
                });

                me.process();
            }
        }
    });

    me.process();

    return Ok(watcher);
}
