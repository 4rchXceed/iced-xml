# Changelog

## Last commit: 0e7de5892fc092e0d5047b1e0b28caa5cbc04d70
~ Better file structure
+ Checkbox element
+ Text styling options: center, font (with Font styling type), and so much more
+ New element: checkbox, alongside with some other styling options
~ Bug fixes

### Commit msg: "~ Better file structure + Checkbox element + Text styling options: center, font (with Font styling type), and so much more + New element: checkbox, alongside with some other styling options ~ Bug fixes"

## Last commit: 13ccb5228ed538c510e9d9cbfa8ee2ca4c17aaa9
+ Added ::virtuals selectors for styling
+ Improved virtual API for elements
+ Added styling flag API
- Removed test_implementation (useless)
+ Added link to personal roadmap
+ Benchmark tests

### Commit msg: "+ Added ::virtuals selectors for styling + Improved virtual API for elements + Added styling flag API ~ Change test_implementation to test some other things + Added link to personal roadmap + Benchmark tests"

## Last commit: 2e98fe9d3c1657f1a16a61e878fb14cb75f4d0cc
+ ComboBox element (called Select)
+ Added new styling options for elements
+ Brand new API for elements: ::for(xyz), allows multiple themes for the same element
+ Fixed some bugs
+ Added missing styling options for elements

### Commit msg: "+ ComboBox element (called Select) + Added new styling options for elements + Brand new API for elements: ::for(xyz), allows multiple themes for the same element + Fixed some bugs + Added missing styling options for elements"

## Last commit: 50b76be4c53f53e2ed651e66cbb2e6b43b49b12c
~ Move all element_renderer logic to a separate file
~ Renamed container (old object) to col (aka column)
+ Added new element: Container (only 1 child allowed)

### Commit msg: "~ Move all element_renderer logic to a separate file ~ Renamed container (old object) to col (aka column) + Added new element: Container (only 1 child allowed)"

## Last commit: 3b1bef719c70d308465b014456b6eeba769249a2
+ Added new element: float
~ Fixed a bug with padding
~ Fixed a bug on XML style inheritance
+ Added new element: grid (!! requires grid-responsive-width to work properly)

### Commit msg: "+ Added new element: float ~ Fixed a bug with padding ~ Fixed a bug on XML style inheritance + Added new element: grid (!! requires grid-responsive-width to work properly)"

## Last commit: 3784f9e5c667e85934b498c826eb1cce0cc0d8b5
+ Added new element: WindowSystem: a pane grid system, allows for multiple windows to be opened at once, and supports window dragging and resizing
+ Added elements / code related to the WindowSystem element
+ Added new EmitEvent system for DOM -> Element communication

### Commit msg: "+ Added new element: WindowSystem: a pane grid system, allows for multiple windows to be opened at once, and supports window dragging and resizing + Added elements / code related to the WindowSystem element + Added new EmitEvent system for DOM -> Element communication. View CHANGELOGS.md for more details"

## Last commit: 0f1cfe212ecdb9da21353403babb2486191e9bfc
+ Added new element: radio
~ Structure change for radio element:
  + stringdb -> id to string database
  + Added RendererEvent for the elements to modify the renderer

### Commit msg: "+ Added new element: radio ~ Structure change for radio element: + stringdb -> id to string database + Added RendererEvent for the elements to modify the renderer"

## Last commit: 3784f9e5c667e85934b498c826eb1cce0cc0d8b5
+ Added new element scrollable
~ Structure change for scrollable element:
  + New property `scrollable_scroll_state` in EventResponse
~ Some theming changes for the new element

### Commit msg: "+ Added new element scrollable ~ Structure change for scrollable element:   + New property `scrollable_scroll_state` in EventResponse ~ Some theming changes for the new element"

## Last commit: 6f8a27a81060575ca49d3b23eafd8bed8752f76e
+ Added new element: progress
~ Renamed text_color to foreground_color
+ Added support for <Element /> self-closing tags
~ Some theming changes for the new element

### Commit msg: "+ Added new element: progress ~ Renamed text_color to foreground_color + Added support for <Element /> self-closing tags ~ Some theming changes for the new element"

## Last commit: b9809a989b21daf5287ceec8670667f810c333a2
+ Added new element: trigger
+ Added new eventresponse data: VectorWH
+ Added new type: VectorWH (width, height)
### Commit msg: "+ Added new element: trigger + Added new eventresponse data: VectorWH + Added new type: VectorWH (width, height)"

## Last commit: c77804890f61584c82217963588f508867842456
+ Added new element: slider
+ Added: slider-height, slider-rail-width and slider-handle-shape (slider-handle-shape has a parser) to the styling system
+ Added code quality disclaimer
### Commit msg: "+ Added new element: slider + Added: slider-height, slider-rail-width and slider-handle-shape (slider-handle-shape has a parser) to the styling system + Added code quality disclaimer"

## Last commit: 3a7e2e1db3f8813492915635483d57852f7c24b4
Ok, big commit here.
+ Added new Windowing System: allows for multiple windows to be opened at once
+ Two new query builder function: open_window() and close_window()
+ Added the worst code I ever wrote: the window_system macro. No seriously I need to change alot of things, but I don't have the time to do it right now. So for now, this is what we have.
~ Refactored the app_wrapper to window_wrapper
~ Updated the example to use the new windowing system
~ Changed to UID provider to be a global static variable
~ Other smaller changes to follow the new windowing system

### Commit msg: "+ Added new Windowing System: allows for multiple windows to be opened at once + Two new query builder function: open_window() and close_window() + Added the window_system macro: allows for way simpler usage of the windowing system. Needs some improvements ~ Refactored the app_wrapper to window_wrapper ~ Updated the example to use the new windowing system ~ Changed to UID provider to be a global static variable ~ Other smaller changes to follow the new windowing system"

## Last commit: 7a86da1739725a2e028b67a47d6b36424a583b77
+ New element: space (just space, nothing else)
~ Changed text for safe_read_file
~ Changed examples to match new structure

### Commit msg: "+ New element: space (just space, nothing else) ~ Changed text for safe_read_file ~ Changed examples to match new structure"

## Last commit: 6529cbcb6460ef4b2ee8004bc59927ee276ed118
+ New element: table (a table that renders datas)
+ New element: var (a variable that can be used to render table's data)
~ Added new system to pass datas to child elements (like table and var)
~ Fixed missing styles for <Center /> element
~ Other small changes for the new table element

### Commit msg: "+ New element: table (a table that renders datas) + New element: var (a variable that can be used to render table's data) ~ Added new system to pass datas to child elements (like table and var) ~ Fixed missing styles for <Center /> element ~ Other small changes for the new table element"

## Last commit: 628acfeb5e507d8d72febf37b2d211a7794ba9e7
+ New element: input
~ Fixed bug in center + added slider to the mod.rs (was removed for some reason)
+ Added center-type styling option for <Center /> element (align/center)
+ Added (kind-of) input-icon styling option, same var as select-icon, but different name for clarity

### Commit msg: "+ New element: input ~ Fixed bug in center + added slider to the mod.rs (was removed for some reason) + Added center-type styling option for <Center /> element (align/center) + Added (kind-of) input-icon styling option, same var as select-icon, but different name for clarity"

## Last commit: d76ed823949fb12726c65d220740dc827a04a92b
Big commit here.
+ New selector type: Complex, allows for:
  - > (child selector)
  - " " (descendant selector)
  - ~ (sibling selector) [no +, all siblings are selected no matter before or after]
  - Tag#id.class (more specific selector)
+ New enum: ComplexQueryJoinType (Descendant, Child, Silbling, Also)
+ New struct: ComplexQuery, allows for complex queries to be built, recursive
~ Updated parse_selector to support complex queries
+ New CSS parsing function: split_complex_selector
~ Updated: init_element_from_xml: new argument (mandatory): `parent_uid: i32`
~ Updated elements to support `parent_uid: i32` argument
~ Updated element renderer to support parent-child relationships, and to support complex queries
+ New utility: `is_alphabetic` (checks if a string is alphabetic)
+ New DOM element API: Dom::query_selector, allows for complex queries to be used on the DOM. DOESN'T SUPPORT `,`!!

### Commit msg: "Added complex selector support"
### Other: -m "Big commit here." -m "+ New selector type: Complex, allows for:" -m "  - > (child selector)" -m "  - " " (descendant selector)" -m "  - ~ (sibling selector) [no +, all siblings are selected no matter before or after]" -m "  - Tag#id.class (more specific selector)" -m "+ New enum: ComplexQueryJoinType (Descendant, Child, Silbling, Also)" -m "+ New struct: ComplexQuery, allows for complex queries to be built, recursive" -m "~ Updated parse_selector to support complex queries" -m "+ New CSS parsing function: split_complex_selector" -m "~ Updated: init_element_from_xml: new argument (mandatory): `parent_uid: i32`" -m "~ Updated elements to support `parent_uid: i32` argument" -m "~ Updated element renderer to support parent-child relationships, and to support complex queries" -m "+ New utility: `is_alphabetic` (checks if a string is alphabetic)" -m "+ New DOM element API: Dom::query_selector, allows for complex queries to be used on the DOM. DOESN'T SUPPORT `,`!!"

## Last commit: 0fad054e6ba01de0c263f9fc3fbe80f0041edd36
+ Renamed background_color to background
+ Added background parser with gradient support
+ Retyped background from Color to Background
~ Updated the elements to use the new background system
+ Added to_rad util
+ Added fg-elem styling option
+ Added parse_color_op parser

### Commit msg: "Gradient background support"
### Other: -m "+ Renamed background_color to background" -m "Added background parser with gradient support" -m "Retyped background from Color to Background" -m "Updated the elements to use the new background system" -m "Added to_rad util" -m "Added fg-elem styling option" -m "Added parse_color_op parser"
