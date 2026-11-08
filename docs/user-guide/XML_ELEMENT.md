# XmlElement struct
To generate elements from Rust, you can use two methods:

## From a string
- `xml("<Element />")`
With it's variants:
- `try_xml("")?;`
- and the `_fonts` variants, if you apply a custom Font:
  - `xml_fonts("", fonts)`
  - `try_xml_fonts("", fonts)`

## From an `XmlElement::void()` element
You can generate en empty XmlElement via it's void method, and then modify attributes / children via other methods.

Here's a list of them (from the iced-xml's code):
### impl XmlElement

#### pub fn virt() -> Self

Creates a virtual XmlElement

#### pub fn void() -> Self

Creates an empty XmlElement, that will be of size 0x0

#### pub fn set_tag(&mut self, tag: &str)

Sets the tag of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.set_tag("Div");
```

#### pub fn set_text(&mut self, text: &str)

Sets the text content of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.set_text("Hello World");
```

#### pub fn set_attributes(&mut self, attributes: HashMap<String, String>) 

Sets the attributes of the XmlElement. Examples:

```rust
use std::collections::HashMap;
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
let mut attributes = HashMap::new();
attributes.insert(String::from("id"), String::from("my-id"));
element.set_attributes(attributes);
```

#### pub fn set_children(&mut self, children: Vec<Box\<XmlElement\>>)

Sets the children of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
let child = iced_xml::xml_struct::xml_element::XmlElement::void();
element.set_children(vec![Box::new(child)]);
```

#### pub fn apply_css(&mut self, key: &str, value: &str)

Adds a rule the theme of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.apply_css("bg", "red");
```

#### pub fn append_child(&mut self, child: XmlElement)

Appends a child to the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
let child = iced_xml::xml_struct::xml_element::XmlElement::void();
element.append_child(child);
```

#### pub fn remove_child(&mut self, index: usize)

Removes a child from the XmlElement by index. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
let child = iced_xml::xml_struct::xml_element::XmlElement::void();
element.append_child(child);
element.remove_child(0);
```

#### pub fn children_count(&self) -> usize

Returns the number of children of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
let child = iced_xml::xml_struct::xml_element::XmlElement::void();
element.append_child(child);
assert_eq!(element.children_count(), 1);
```

#### pub fn set_id(&mut self, id: &str)

Sets the id of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.set_id("my-id");
```

#### pub fn clear_id(&mut self)

Clears the id of the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.set_id("my-id");
element.clear_id();
```

#### pub fn add_class(&mut self, class: &str)

Adds a class to the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.add_class("my-class");
```

#### pub fn remove_class(&mut self, class: &str)

Removes a class from the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.add_class("my-class");
element.remove_class("my-class");
```

#### pub fn add_data(&mut self, key: &str, value: &str)

Adds a data attribute to the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.add_data("key", "value");
```

#### pub fn remove_data(&mut self, key: &str)

Removes a data attribute from the XmlElement. Examples:

```rust
let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
element.add_data("key", "value");
element.remove_data("key");
```

## Where to use `XmlElement`?
You can use XmlElement in two Dom Queries:

### `replace(Box<...>)`
Replaces an element by another

Example (from [FFSearch.rs](https://git.webdash.dev/ArchXceed/FFSearch.rs/src/commit/3288ddcd8f65f25d32e49ad329016eecfc05225b/crates/frontends/ui/src/gui/windows/tasks.rs#L137), one of my projects)
```rust
self.qb.b(Dom::get_element_by_id("tasks-list").replace(Box::new(task_list)));

self.process();
```

### `get_element()`
Gets the XmlElement for a element

I don't have any examples :(
