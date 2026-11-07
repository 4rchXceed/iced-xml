# Window ID
The window ID is supposed to represent a window instance unique id.
This is a u32, and is managed by Iced.

You need this when closing windows.

## Example
Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/c29d3bb97e6352f4c867003809d4af10921ab60e/crates/frontends/ui/src/gui/params.rs#L4), one of my projects)

```rust
self.qb
    .b(Dom::get_element_by_id("next").add_event_listener(EventListenerTypes::Click))
    .with_callback(|this, _, app| {
        match this.current_panel {
            Panels::Welcome => {
                this.qb.b(Dom::get_element_by_id("next").set_property("text", "Finish"));

                this.process();

                this.current_panel = Panels::HowItWorks;
            }
            Panels::HowItWorks => {
                // Here, we pass this.me_winid as 1st param
                this.qb.close_window(this.me_winid, app);
                return;
            }
        }

        this.render_panel();
    });
```
