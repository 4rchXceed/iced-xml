use std::collections::HashMap;

use iced::{
    Border, Color, Length,
    widget::{self, button, row},
};

use crate::{
    dom::types::{
        CustomElementEvent, DomInternalMessageType, EventListenerTypes, EventResponse,
        QueryResponse,
    },
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener},
    },
    parse_utils::parsers::pane_axis::parse_pane_axis,
    rs_utils::{HashableF32, HashableGridTarget},
    xml_engine::Message,
    xml_struct::{
        elements::{
            element_base::ElementBase, library::ElementError, window_system::PaneType::InternalPane,
        },
        xml_element::XmlElement,
    },
};

#[derive(Debug, Clone, Hash)]
pub struct TWMWindowOpenParams {
    pub window_id: String,
    pub parent_window_id: String,
    pub split_method: String,
}

#[derive(Debug, Clone)]
struct WindowChild {
    content_uid: i32,
    fullscreen_btn: Option<i32>,
    close_btn: Option<i32>,
    titlebar_content_uid: Option<i32>,
}

#[derive(Debug, Clone)]
enum PaneType {
    InternalPane(String, WindowChild), // Child
}

impl PaneType {
    // fn to_pane_id(&self) -> i32 {
    //     match self {
    //         PaneType::InternalPane(id) => *id,
    //     }
    // }
}

pub struct WindowSystem {
    children_id: HashMap<String, WindowChild>,
    focused: iced::widget::pane_grid::Pane,
    panes: iced::widget::pane_grid::State<PaneType>,
    drag_border_size: f32,
}

impl WindowSystem {
    fn window_id_to_pane(&self, window_id: &str) -> Option<iced::widget::pane_grid::Pane> {
        if window_id == "focused" {
            return Some(self.focused);
        }

        for (pane, pane_type) in self.panes.iter() {
            match pane_type {
                PaneType::InternalPane(id, _) => {
                    if id == window_id {
                        return Some(*pane);
                    }
                }
            }
        }
        return None;
    }

    fn preprocess_pane_drag(
        &self,
        ev: iced::widget::pane_grid::DragEvent,
        me: i32,
    ) -> EventResponse {
        let drag_type;
        let data_window;

        let mut data_target: Option<iced::widget::pane_grid::Target> = None;
        match ev {
            widget::pane_grid::DragEvent::Canceled { pane } => {
                data_window = Some(pane);
                drag_type = String::from("canceled");
            }
            widget::pane_grid::DragEvent::Picked { pane } => {
                data_window = Some(pane);
                drag_type = String::from("picked");
            }
            widget::pane_grid::DragEvent::Dropped { pane, target } => {
                data_window = Some(pane);
                drag_type = String::from("dropped");
                data_target = Some(target);
            }
        };

        let mut ev_res = EventResponse::new(me, EventListenerTypes::Drag);

        ev_res.data_str = Some(drag_type);
        ev_res.window_system_data_window = data_window;

        if let Some(target) = data_target {
            ev_res.window_system_data_target = Some(HashableGridTarget::new(target));
        }

        return ev_res;
    }
}

fn preprocess_pane_state(
    state: &mut iced::widget::pane_grid::State<PaneType>,
    renderer: &mut ElementMappings,
    elements: Vec<Box<XmlElement>>,
    parent_pane: iced::widget::pane_grid::Pane,
    self_uid: i32,
) -> Result<HashMap<String, WindowChild>, ElementError> {
    let mut children: HashMap<String, WindowChild> = HashMap::new();

    for child in elements {
        if child.tag == "Window" {
            if let Some(child_id) = child.attributes.get("window-id") {
                let child_uid = renderer.init_element_from_xml(content(child.clone())?, self_uid);

                let fullscreen_btn = maximize_button(child.clone())
                    .map(|btn| renderer.init_element_from_xml(btn, self_uid));

                let close_btn = close_button(child.clone())
                    .map(|btn| renderer.init_element_from_xml(btn, self_uid));

                let titlebar_content_uid = titlebar_content(child.clone())
                    .map(|btn| renderer.init_element_from_xml(btn, self_uid));

                let child_datas = WindowChild {
                    content_uid: child_uid,
                    fullscreen_btn: fullscreen_btn,
                    close_btn: close_btn,
                    titlebar_content_uid,
                };

                children.insert(child_id.clone(), child_datas.clone());

                if child.children.len() > 0
                    && child.attributes.get("window-closed").is_none()
                    && let Some(split_method) = child.attributes.get("split-method")
                {
                    let pane = state.split(
                        parse_pane_axis(split_method.as_str())
                            .unwrap_or(widget::pane_grid::Axis::Horizontal),
                        parent_pane,
                        InternalPane(child_id.clone(), child_datas),
                    );

                    if let Some(pane) = pane {
                        preprocess_pane_state(
                            state,
                            renderer,
                            window_children(child)?,
                            pane.0,
                            self_uid,
                        )?;
                    }
                }
            } else {
                return Err(ElementError::WindowAttributesMissing(child));
            }
        }
    }

    return Ok(children);
}

fn titlebar(element: Box<XmlElement>) -> Option<Box<XmlElement>> {
    for child in &element.children {
        if child.tag == "WindowTitlebar" {
            return Some(child.clone());
        }
    }

    return None;
}

fn maximize_button(element: Box<XmlElement>) -> Option<Box<XmlElement>> {
    let titlebar = titlebar(element)?;

    for child in &titlebar.children {
        if child.tag == "MaximizeButton" {
            return Some(child.clone());
        }
    }

    return None;
}

fn close_button(element: Box<XmlElement>) -> Option<Box<XmlElement>> {
    let titlebar = titlebar(element)?;

    for child in &titlebar.children {
        if child.tag == "CloseButton" {
            return Some(child.clone());
        }
    }
    return None;
}

fn titlebar_content(element: Box<XmlElement>) -> Option<Box<XmlElement>> {
    let titlebar = titlebar(element)?;

    for child in &titlebar.children {
        if child.tag == "Content" {
            return Some(child.clone());
        }
    }

    return None;
}

fn content(element: Box<XmlElement>) -> Result<Box<XmlElement>, ElementError> {
    for child in &element.children {
        if child.tag == "WindowContent" {
            return Ok(child.clone());
        }
    }

    return Err(ElementError::WindowMustHaveWindowContent(element));
}

fn window_children(element: Box<XmlElement>) -> Result<Vec<Box<XmlElement>>, ElementError> {
    let mut childs: Vec<Box<XmlElement>> = Vec::new();

    for child in &element.children {
        if child.tag == "WindowChildren" {
            for grandchild in &child.children {
                if grandchild.tag != "Window" {
                    return Err(ElementError::WindowChildrenNotWindowAsChild(element));
                }

                childs.push(grandchild.clone());
            }
        }
    }

    return Ok(childs);
}

fn transparent_btn_style() -> iced::widget::button::Style {
    button::Style {
        background: None,
        text_color: Color::BLACK,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
        },
        shadow: iced::Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector { x: 0.0, y: 0.0 },
            blur_radius: 0.0,
        },
        snap: false,
    }
}

impl ElementBase for WindowSystem {
    fn new(
        xml_element: Box<XmlElement>,
        renderer: &mut ElementMappings,
        self_uid: i32,
    ) -> Result<Self, ElementError> {
        // If it supports children, initialize them here with renderer.init_element
        let first_child = xml_element
            .children
            .get(0)
            .ok_or(ElementError::WindowSystemElementMustHaveOneChild(
                xml_element.clone(),
            ))?
            .clone();

        if first_child.tag != "Window" {
            return Err(ElementError::WindowSystemChildMustBeWindow(
                xml_element.clone(),
                first_child.clone(),
            ));
        }

        let mut drag_border_size = 5.0;
        if let Some(drag_border_size_str) = xml_element.attributes.get("drag-border-size") {
            drag_border_size = drag_border_size_str.parse::<f32>().unwrap_or(5.0);
        }

        let first_window_content = content(first_child.clone())?;
        let first_window_children = window_children(first_child.clone())?;

        let first_uid = renderer.init_element_from_xml(first_window_content, self_uid);
        let first_id = first_child
            .attributes
            .get("window-id")
            .ok_or(ElementError::WindowAttributesMissing(xml_element))?;

        let first_child = WindowChild {
            content_uid: first_uid,
            fullscreen_btn: maximize_button(first_child.clone())
                .map(|btn| renderer.init_element_from_xml(btn, self_uid)),
            close_btn: close_button(first_child.clone())
                .map(|btn| renderer.init_element_from_xml(btn, self_uid)),
            titlebar_content_uid: titlebar_content(first_child.clone())
                .map(|btn| renderer.init_element_from_xml(btn, self_uid)),
        };

        let mut state = iced::widget::pane_grid::State::new(InternalPane(
            first_id.clone(),
            first_child.clone(),
        ));

        let mut children = preprocess_pane_state(
            &mut state.0,
            renderer,
            first_window_children,
            state.1,
            self_uid,
        )?;

        children.insert(first_id.clone(), first_child);

        return Ok(Self {
            children_id: children.clone(),
            panes: state.0, // Initialize panes as empty
            focused: state.1,
            drag_border_size: drag_border_size,
        });
    }

    fn render<'a>(
        &'a self,
        renderer: &'a ElementsManager,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut hover_region = datas.default_theme.clone();
        let mut hover_line = datas.default_theme.clone();
        let mut picked_line = datas.default_theme.clone();

        if let Some(hover_region_style) = datas.flag_themes.get("window-system-drop-region") {
            hover_region = hover_region_style.clone();
        }
        if let Some(hover_line_style) = datas.flag_themes.get("window-system-drop-border") {
            hover_line = hover_line_style.clone();
        }
        if let Some(picked_line_style) = datas.flag_themes.get("window-system-drag-border") {
            picked_line = picked_line_style.clone();
        }

        let mut pane_grid: iced::widget::pane_grid::PaneGrid<'a, Message> =
            iced::widget::pane_grid::PaneGrid::new(&self.panes, |pane, state, is_fullscreen| {
                match state {
                    InternalPane(_, child_datas) => {
                        let mut btn_max = button(iced::widget::text("Max"));
                        let mut btn_close = button(iced::widget::text("Close"));

                        if let Some(fullscreen_btn) = child_datas.fullscreen_btn {
                            btn_max = button(
                                renderer.render_element(fullscreen_btn, datas.child_data.clone()),
                            )
                            .style(|_, _| transparent_btn_style());
                        }

                        if let Some(close_btn) = child_datas.close_btn {
                            btn_close = button(
                                renderer.render_element(close_btn, datas.child_data.clone()),
                            )
                            .style(|_, _| transparent_btn_style());
                        }

                        let mut ev_res_max =
                            EventResponse::new(self_uid, EventListenerTypes::Maximize);
                        let ev_res_restore =
                            EventResponse::new(self_uid, EventListenerTypes::Restore);
                        let mut ev_res_close =
                            EventResponse::new(self_uid, EventListenerTypes::Close);

                        ev_res_max.window_system_data_window = Some(pane);
                        ev_res_close.window_system_data_window = Some(pane);

                        btn_max = btn_max.on_press(Message::DomEvent(None, ev_res_max.clone()));

                        if is_fullscreen {
                            btn_max =
                                btn_max.on_press(Message::DomEvent(None, ev_res_restore.clone()));
                        } else {
                            btn_close =
                                btn_close.on_press(Message::DomEvent(None, ev_res_close.clone()));
                        }

                        let mut title_bar_content: iced::Element<'a, Message> =
                            iced::widget::text("").into();

                        if let Some(titlebar_content) = child_datas.titlebar_content_uid {
                            title_bar_content =
                                renderer.render_element(titlebar_content, datas.child_data.clone());
                        }

                        let title_bar = iced::widget::pane_grid::TitleBar::new(title_bar_content)
                            .controls(iced::widget::pane_grid::Controls::new(
                                row![btn_max, btn_close].spacing(5).width(Length::Shrink),
                            ))
                            .padding(10);

                        let element = renderer
                            .render_element(child_datas.content_uid, datas.child_data.clone());

                        return iced::widget::pane_grid::Content::new(element).title_bar(title_bar);
                    }
                }
            });

        let me = self_uid.clone();

        pane_grid = pane_grid
            .height(theme.height)
            .min_size(theme.pane_min_size)
            .spacing(theme.spacing)
            .style(move |_| iced::widget::pane_grid::Style {
                hovered_region: widget::pane_grid::Highlight {
                    background: hover_region.background,
                    border: Border {
                        color: hover_region.border_color,
                        width: hover_region.border_width,
                        radius: hover_region.border_radius,
                    },
                },
                hovered_split: widget::pane_grid::Line {
                    color: hover_line.border_color,
                    width: hover_line.border_width,
                },
                picked_split: widget::pane_grid::Line {
                    color: picked_line.border_color,
                    width: picked_line.border_width,
                },
            })
            .width(theme.width)
            .on_click(move |p| {
                let mut res = EventResponse::new(me, EventListenerTypes::Focus);

                res.window_system_data_window = Some(p);

                Message::DomEvent(None, res)
            })
            .on_drag(move |ev| Message::DomEvent(None, self.preprocess_pane_drag(ev, me)))
            .on_resize(self.drag_border_size, move |ev| {
                let mut res = EventResponse::new(me, EventListenerTypes::Resize);

                res.window_system_data_split = Some(ev.split);
                res.data_float = Some(HashableF32::new(ev.ratio));

                Message::DomEvent(None, res)
            });

        // Register any events here
        for event in events {
            match event.event_type {
                EventListenerTypes::Focus => {
                    pane_grid = pane_grid.on_click(move |pane| {
                        let mut res = EventResponse::new(me, event.event_type.clone());

                        if let Some(internal_pane) = self.panes.get(pane) {
                            let pane_id = match internal_pane {
                                InternalPane(pane_id, _) => pane_id.clone(),
                            };

                            res.data_str = Some(pane_id);
                            res.window_system_data_window = Some(pane);

                            Message::DomEvent(Some(event.event_uid), res)
                        } else {
                            Message::DomEvent(Some(event.event_uid), res)
                        }
                    });
                }
                EventListenerTypes::Drag => {
                    pane_grid = pane_grid.on_drag(move |ev| {
                        Message::DomEvent(Some(event.event_uid), self.preprocess_pane_drag(ev, me))
                    })
                }
                EventListenerTypes::Resize => {
                    pane_grid = pane_grid.on_resize(self.drag_border_size, move |ev| {
                        let mut res = EventResponse::new(me, event.event_type.clone());

                        res.window_system_data_split = Some(ev.split);
                        res.data_float = Some(HashableF32::new(ev.ratio));

                        Message::DomEvent(Some(event.event_uid), res)
                    })
                }
                _ => (),
            }
        }

        return pane_grid.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::FireEvent(event) => match event {
                CustomElementEvent::TWMMaximizeWindow(window_id) => {
                    let pane = self.window_id_to_pane(window_id);
                    if let Some(pane) = pane {
                        self.panes.maximize(pane);

                        Some(ElementEventResponse::success())
                    } else {
                        Some(ElementEventResponse::new(QueryResponse::fail(
                            format!("Pane {} not found", window_id).as_str(),
                        )))
                    }
                }
                CustomElementEvent::TWMRestoreWindow() => {
                    self.panes.restore();
                    Some(ElementEventResponse::success())
                }
                CustomElementEvent::TWMCloseWindow(window_id) => {
                    let window = self.window_id_to_pane(window_id);

                    if let Some(window) = window {
                        let sibling_op = self.panes.close(window);

                        if let Some(sibling) = sibling_op {
                            let (_, to_focus) = sibling;
                            self.focused = to_focus;
                        }

                        Some(ElementEventResponse::success())
                    } else {
                        Some(ElementEventResponse::new(QueryResponse::fail(
                            format!("Pane {} not found", window_id).as_str(),
                        )))
                    }
                }
                CustomElementEvent::TWMFocusWindow(window) => {
                    let pane = self.window_id_to_pane(window);
                    if let Some(pane) = pane {
                        self.focused = pane;

                        Some(ElementEventResponse::success())
                    } else {
                        Some(ElementEventResponse::new(QueryResponse::fail(
                            format!("Pane {} not found", window).as_str(),
                        )))
                    }
                }
                CustomElementEvent::TWMDragWindow(window_id, target) => {
                    let window = self.window_id_to_pane(window_id);

                    if let Some(window) = window {
                        self.panes.drop(window, target.value());

                        Some(ElementEventResponse::success())
                    } else {
                        Some(ElementEventResponse::new(QueryResponse::fail(
                            format!("Pane {} not found", window_id).as_str(),
                        )))
                    }
                }
                CustomElementEvent::TWMOpenWindow(open_params) => {
                    let window_op = self.children_id.get(&open_params.window_id);

                    if let Some(window) = window_op {
                        let parent_window_op =
                            self.window_id_to_pane(&open_params.parent_window_id);

                        if let Some(parent_window) = parent_window_op {
                            let split_method = parse_pane_axis(&open_params.split_method)
                                .unwrap_or(widget::pane_grid::Axis::Horizontal);

                            let pane = self.panes.split(
                                split_method,
                                parent_window,
                                InternalPane(open_params.window_id.clone(), window.clone()),
                            );

                            if let Some(pane) = pane {
                                self.focused = pane.0;

                                Some(ElementEventResponse::success())
                            } else {
                                Some(ElementEventResponse::new(QueryResponse::fail(
                                    format!(
                                        "Failed to open Pane {} under Parent Pane {}",
                                        open_params.window_id, open_params.parent_window_id
                                    )
                                    .as_str(),
                                )))
                            }
                        } else {
                            return Some(ElementEventResponse::new(QueryResponse::fail(
                                format!("Parent Pane {} not found", open_params.parent_window_id)
                                    .as_str(),
                            )));
                        }
                    } else {
                        Some(ElementEventResponse::new(QueryResponse::fail(
                            format!("Pane {} not found", open_params.window_id).as_str(),
                        )))
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &EventListenerTypes,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type {
            EventListenerTypes::Restore => {
                self.panes.restore();

                return Some(ElementEventResponse::success());
            }
            EventListenerTypes::Resize => {
                if let Some(split) = event_response.window_system_data_split
                    && let Some(size) = &event_response.data_float
                {
                    self.panes.resize(split, size.value());
                }

                return Some(ElementEventResponse::success());
            }
            _ => {}
        }

        if let Some(data_window) = event_response.window_system_data_window {
            match event_type {
                EventListenerTypes::Maximize => {
                    self.panes.maximize(data_window);

                    return Some(ElementEventResponse::success());
                }
                EventListenerTypes::Close => {
                    let datas_op = self.panes.close(data_window);

                    if datas_op.is_some() {
                        let (_, to_focus) = datas_op.unwrap();
                        self.focused = to_focus;
                    }
                    return Some(ElementEventResponse::success());
                }
                EventListenerTypes::Focus => {
                    self.focused = data_window;

                    return Some(ElementEventResponse::success());
                }
                EventListenerTypes::Drag => {
                    if let Some(event_type) = &event_response.data_str {
                        match event_type.as_str() {
                            "dropped" => {
                                if let Some(data_target) = &event_response.window_system_data_target
                                {
                                    self.panes.drop(data_window, data_target.value());
                                }

                                return Some(ElementEventResponse::success());
                            }
                            _ => return Some(ElementEventResponse::success()),
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
