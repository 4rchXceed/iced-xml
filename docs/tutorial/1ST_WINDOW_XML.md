# XML Elements - TodoListWindow
The XML will declare:
- text
- elements (buttons, ...)
- the "structure" of the window

## More infos about XML
More infos can be found in the [user guides](../user-guide/XML_SYNTAX.md)

## What do we want in the XML?
We want (for our TODO):
- A title (html eq: `<h1>`)
- A list of Tasks (will be creating directly in the code, no declaration in the XML needed)
- An input and a button, to add a new Task

## The code
Here's the code for these elements, it's explained below:
```xml
<Col classes="root">
    <Label classes="title">TODO List</Label>
    <Label id="tasks-list">Failed to load TODOs</Label>
    <Row id="new-todo-container">
        <Col classes="new-todo-inputs">
            <Input id="new-todo-name" placeholder="New TODO name"></Input>
            <Textarea id="new-todo-description" placeholder="New TODO description"></Textarea>
        </Col>
        <Button id="new-todo-btn">Add</Button>
    </Row>
</Col>
```

### Explanation
Here's a small explanation:
- `Col`: see [user guide](../user-guide/XML_SYNTAX.md)
- `Label`: see [user guide](../user-guide/XML_SYNTAX.md)
- `Row`: Same as `Col` but it's horizontal
- `Input`: Basically an HTML Input
- `Button`: A Button with text inside

### And the attributes?
See [user guide](../user-guide/XML_SYNTAX.md) for `id` and `classes` attributes

- `placeholder` is the input placeholder

## Next steps
We will then see how to make [queries from Rust](./1ST_WINDOW_RUST.md)
