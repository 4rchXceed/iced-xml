# XML Syntax
The XML declares:
- text
- elements (buttons, ...)
- the "structure" of the window

Any window needs an XML source. The XML File needs to have only one XML Element at root, and then it depends on the element you add

## More infos in elements
TODO

## Example
Here's a basic window that will show a "Hello World" label

```xml
<Label>Hello World</Label>
```

## Attributes
Some elements have attributes, for example the input, with it's placeholder

```xml
<Input placeholder="Enter something...">Default Content</Input>
```

## Special attributes
Some attributes are special, they can define how to get elements, and other informations

### The `id` attribute
With the `id` attribute, you can assign a 100% unique ID for an element

#### XML
Example:
```xml
<Label id="special-label"></Label>
```

#### CSS
To access it on css, you need to do `#id`

Example:
```css
#special-label {
    fg: white;
}
```

#### Rust
And on rust, you need to call `Dom::get_element_by_id`

Example:
```rust
self.qb.b(Dom::get_element_by_id("special-label").remove());

self.process();
```

### The `classes` attribute
The `classes` attribute is like the `id` attribute, but you can set multiple classes per element, and multiple elements can share the same class.

Examples:
#### XML

```xml
<Label classes="fg-white change-text"></Label>
```

#### CSS
To access it on css, you need to do `.class`

```css
.fg-white {
    fg: white;
}
```

#### Rust
And on rust, you need to call `Dom::get_elements_by_class`

Example:
```rust
self.qb.b(Dom::get_elements_by_class("special-label").set_property("text", "Hello World 2"));

self.process();
```

### The `data-` attributes
Can store custom datas.

See [Dom manipulation](./DOM_MANIPULATION.md) section data props to better understand data props

It's used by creating an attribute starting with `data-`.

Example:
```xml
<Label data-newtext="Hello World 3" classes="change-text"></Label>
```

(See [Dom manipulation](./DOM_MANIPULATION.md) for Rust examples)

### The `style:` attributes
These attributes will apply a style to the current element.

Example: if you want to apply a `fg: white` to a label, without CSS, you can do:
```xml
<Label style:fg="white"></Label>
```

## Some very userful elements
To avoid looking through all available elements, here are the top 2 most useful ones:

### Col
A Col is a vertical stack of multiple elements

Example:
```xml
<Col>
    <Element1 />
    <Element2 />
    <Element3 />
</Col>
```

### Label
A Label is a basic text

Example:
```xml
<Label>Element 1</Label>
```

_There are a lot more elements, but these one will be the most useful to you, probably_
