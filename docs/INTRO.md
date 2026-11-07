# Iced-XML

_Note: this documentation is only for the V0.1, I am planning to change a lot of things in the V0.2_

## What is iced-xml?
Iced-XML I a layer on top of the iced library, that allows users to create GUI apps easily.

This allows users to use:
- CSS-like styling
- HTML-like XML elements declaration
- JavaScript-like DOM manipulation

## Details: CSS
The CSS-like styling works like real CSS:
```css
.selector, #other-selector > TagName {
    key: value;
}
```

## Details: HTML (XML)
```xml
<Select id="select" placeholder="Select an option">
    <Option value="1">Option 1</Option>
    <Option value="1" selected="true">Option 2</Option>
    <Option value="1">Option 3</Option>
    <Option value="1">Option 4</Option>
</Select>
```

## JavaScript-like DOM manipulation
```rust
self.qb.b(Dom::get_element_by_id("select").add_event_listener(EventListenerTypes::Click)).with_callback(|this, er, app| {
    if let Some(selector) = er.data_target {
        this.do_action(app.state.shared_data);
    }
}).then(|this, qr| {
    if !qr.success {
        println!("Failed to register the event")
    }
});

self.process();
```

_Note: after **EVERY** action/bunch of actions, you need to call:_

`self.process();` or `this.process();`

_Else the changes won't be applied_

## [Next steps](tutorial/QUICK_START.md)
<!--## [Sections](./SECTIONS.md)-->

## Note on "Why it's bad"
I made this project while learning Rust. This is why I would (and will) make a lot of changes, like... half the project and changing almost everything

### What's "bad" ?
- It's not easy to expend the element list (I didn't use Arc<>)
- It's not easy to catch errors
- Callbacks are always static functions

### So ?
For the V0.2, I want to:
- Have something working more like React
- I also want to drop the "layer" thingy but allow the user to still communicate directly with the base UI library, and I probably won't choose Iced.rs as base
