use notify::RecommendedWatcher;
use notify::{Event, Result, Watcher};
use std::{path::Path, sync::mpsc};

use crate::app_wrapper::AppTemplate;

pub fn watch_css_file<T: AppTemplate<T> + 'static>(
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
                println!("Error while loading CSS: {:?}", event.error_message);
            }
        });
    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(
        Path::new(objects.css_path),
        notify::RecursiveMode::NonRecursive,
    )?;

    objects.qb.set_interval(interval).with_callback(|me, _| {
        let ev = me.get_objects().css_watcher_rx.try_recv();
        if let Ok(ev) = ev {
            if let notify::event::EventKind::Access(ref t) = ev.unwrap().kind {
                if let notify::event::AccessKind::Close(close_type) = t {
                    if let notify::event::AccessMode::Write = close_type {
                        let objects = me.get_objects();
                        objects
                            .qb
                            .import_css(std::fs::read_to_string(&objects.css_path).unwrap(), true)
                            .then(|_, event| {
                                if !event.success {
                                    println!("Error while loading CSS: {:?}", event.error_message);
                                }
                            });
                        me.process();
                    }
                }
            }
        }
    });

    me.process();

    Ok(watcher)
}
