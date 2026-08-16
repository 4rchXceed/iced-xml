use std::collections::HashMap;

use iced::{
    Background, Border, Color, Length,
    widget::{self, button, row},
};

// Copy-paste template
use crate::{
    dom::query::{EventResponse, QueryResponse},
    parse_utils::parse_pane_axis,
    rs_utils::{HashableF32, HashableGridTarget},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::{element_base::ElementBase, window_system::PaneType::InternalPane},
        parser::{XmlChangeEvent, XmlElement},
    },
};

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
        let mut ev_res = EventResponse::new(me, String::from("drag"));
        ev_res.data_str = Some(drag_type);
        ev_res.window_system_data_window = data_window;
        if data_target.is_some() {
            ev_res.window_system_data_target = Some(HashableGridTarget::new(data_target.unwrap()));
        }
        return ev_res;
    }
}

fn preprocess_pane_state(
    state: &mut iced::widget::pane_grid::State<PaneType>,
    renderer: &mut ElementRenderer,
    elements: Vec<XmlElement>,
    parent_pane: iced::widget::pane_grid::Pane,
) -> HashMap<String, WindowChild> {
    let mut children: HashMap<String, WindowChild> = HashMap::new();
    for child in elements {
        if child.tag == "Window" {
            if child.attributes.get("window-id").is_none()
                || (child.attributes.get("window-closed").is_none()
                    && child.attributes.get("split-method").is_none())
            {
                panic!("Window element must have window-id and split-method attributes."); // Note: maybe I'm going to make split-method in styling instead of attribute. We'll see
            }
            let child_id = child.attributes.get("window-id").unwrap().clone();
            let child_uid = renderer.init_element_from_xml(&content(&child));
            let fullscreen_btn =
                maximize_button(&child).map(|btn| renderer.init_element_from_xml(&btn));
            let close_btn = close_button(&child).map(|btn| renderer.init_element_from_xml(&btn));
            let titlebar_content_uid =
                titlebar_content(&child).map(|btn| renderer.init_element_from_xml(&btn));
            let child_datas = WindowChild {
                content_uid: child_uid,
                fullscreen_btn: fullscreen_btn,
                close_btn: close_btn,
                titlebar_content_uid,
            };
            children.insert(child_id.clone(), child_datas.clone());
            if child.children.len() > 0 && child.attributes.get("window-closed").is_none() {
                let pane = state.split(
                    parse_pane_axis(child.attributes.get("split-method").unwrap().as_str()),
                    parent_pane,
                    InternalPane(child_id, child_datas),
                );
                if pane.is_some() {
                    preprocess_pane_state(
                        state,
                        renderer,
                        window_children(&child),
                        pane.unwrap().0,
                    );
                }
            }
        }
    }
    return children;
}

fn titlebar(element: &XmlElement) -> Option<XmlElement> {
    for child in &element.children {
        if child.tag == "WindowTitlebar" {
            return Some(child.clone());
        }
    }
    return None;
}

fn maximize_button(element: &XmlElement) -> Option<XmlElement> {
    let titlebar_op = titlebar(element);
    if titlebar_op.is_none() {
        return None;
    }
    let titlebar = titlebar_op.unwrap();
    for child in &titlebar.children {
        if child.tag == "MaximizeButton" {
            return Some(child.clone());
        }
    }
    return None;
}

fn close_button(element: &XmlElement) -> Option<XmlElement> {
    let titlebar_op = titlebar(element);
    if titlebar_op.is_none() {
        return None;
    }
    let titlebar = titlebar_op.unwrap();
    for child in &titlebar.children {
        if child.tag == "CloseButton" {
            return Some(child.clone());
        }
    }
    return None;
}

fn titlebar_content(element: &XmlElement) -> Option<XmlElement> {
    let titlebar_op = titlebar(element);
    if titlebar_op.is_none() {
        return None;
    }
    let titlebar = titlebar_op.unwrap();
    for child in &titlebar.children {
        if child.tag == "Content" {
            return Some(child.clone());
        }
    }
    return None;
}

fn content(element: &XmlElement) -> XmlElement {
    let mut content: Option<XmlElement> = None;
    for child in &element.children {
        if child.tag == "WindowContent" {
            content = Some(child.clone());
            break;
        }
    }
    if content.is_none() {
        panic!("Window element must have a WindowContent child element.");
    }
    return content.unwrap();
}

fn window_children(element: &XmlElement) -> Vec<XmlElement> {
    let mut childs: Vec<XmlElement> = Vec::new();
    for child in &element.children {
        if child.tag == "WindowChildren" {
            for grandchild in &child.children {
                if grandchild.tag != "Window" {
                    panic!("WindowChildren element must have only Window child elements.");
                }
                childs.push(grandchild.clone());
            }
        }
    }
    return childs;
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
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, _: i32) -> Self {
        // If it supports children, initialize them here with renderer.init_element
        if xml_element.children.len() != 1 {
            panic!("WindowSystem element must have exactly one child element.");
        }
        if xml_element.children[0].tag != "Window" {
            panic!("WindowSystem element's child must be a Window element.");
        }

        let mut drag_border_size = 5.0;
        if let Some(drag_border_size_str) = xml_element.attributes.get("drag-border-size") {
            drag_border_size = drag_border_size_str.parse::<f32>().unwrap_or(5.0);
        }

        let first_window_content = content(&xml_element.children[0]);
        let first_window_children = window_children(&xml_element.children[0]);

        let first_uid = renderer.init_element_from_xml(&first_window_content);
        let first_id = xml_element.children[0]
            .attributes
            .get("window-id")
            .unwrap()
            .clone();
        let first_child = WindowChild {
            content_uid: first_uid,
            fullscreen_btn: maximize_button(&xml_element.children[0])
                .map(|btn| renderer.init_element_from_xml(&btn)),
            close_btn: close_button(&xml_element.children[0])
                .map(|btn| renderer.init_element_from_xml(&btn)),
            titlebar_content_uid: titlebar_content(&xml_element.children[0])
                .map(|btn| renderer.init_element_from_xml(&btn)),
        };

        let mut state = iced::widget::pane_grid::State::new(InternalPane(
            first_id.clone(),
            first_child.clone(),
        ));

        let mut children =
            preprocess_pane_state(&mut state.0, renderer, first_window_children, state.1);

        children.insert(first_id, first_child);

        Self {
            children_id: children.clone(),
            panes: state.0, // Initialize panes as empty
            focused: state.1,
            drag_border_size: drag_border_size,
        }
    }

    fn render<'a>(
        &'a self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut hover_region = datas.default_theme.clone();
        let mut hover_line = datas.default_theme.clone();
        let mut picked_line = datas.default_theme.clone();

        let hover_region_style = datas.flag_themes.get("window-system-drop-region");
        let hover_line_style = datas.flag_themes.get("window-system-drop-border");
        let picked_line_style = datas.flag_themes.get("window-system-drag-border");

        if hover_region_style.is_some() {
            hover_region = hover_region_style.unwrap().clone();
        }
        if hover_line_style.is_some() {
            hover_line = hover_line_style.unwrap().clone();
        }
        if picked_line_style.is_some() {
            picked_line = picked_line_style.unwrap().clone();
        }

        let mut pane_grid: iced::widget::pane_grid::PaneGrid<'a, Message> =
            iced::widget::pane_grid::PaneGrid::new(&self.panes, |pane, state, is_fullscreen| {
                match state {
                    InternalPane(_, child_datas) => {
                        let mut btn_max = button(iced::widget::text("Max"));
                        let mut btn_close = button(iced::widget::text("Close"));
                        if child_datas.fullscreen_btn.is_some() {
                            btn_max = button(renderer.render_element(
                                child_datas.fullscreen_btn.unwrap(),
                                datas.child_data.clone(),
                            ))
                            .style(|_, _| transparent_btn_style());
                        }
                        if child_datas.close_btn.is_some() {
                            btn_close = button(renderer.render_element(
                                child_datas.close_btn.unwrap(),
                                datas.child_data.clone(),
                            ))
                            .style(|_, _| transparent_btn_style());
                        }
                        let mut ev_res_max = EventResponse::new(self_uid, String::from("maximize"));
                        let ev_res_restore = EventResponse::new(self_uid, String::from("restore"));
                        let mut ev_res_close = EventResponse::new(self_uid, String::from("close"));
                        ev_res_max.window_system_data_window = Some(pane);
                        ev_res_close.window_system_data_window = Some(pane);
                        btn_max = btn_max.on_press(Message::DomEvent(-1, ev_res_max.clone()));
                        if is_fullscreen {
                            btn_max =
                                btn_max.on_press(Message::DomEvent(-1, ev_res_restore.clone()));
                        } else {
                            btn_close =
                                btn_close.on_press(Message::DomEvent(-1, ev_res_close.clone()));
                        }
                        let mut title_bar_content: iced::Element<'a, Message> =
                            iced::widget::text("").into();
                        if child_datas.titlebar_content_uid.is_some() {
                            title_bar_content = renderer.render_element(
                                child_datas.titlebar_content_uid.unwrap(),
                                datas.child_data.clone(),
                            );
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
                    background: Background::Color(hover_region.background_color),
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
                let mut res = EventResponse::new(me, String::from("click"));
                res.window_system_data_window = Some(p);
                Message::DomEvent(-1, res)
            })
            .on_drag(move |ev| Message::DomEvent(-1, self.preprocess_pane_drag(ev, me)))
            .on_resize(self.drag_border_size, move |ev| {
                let mut res = EventResponse::new(me, String::from("resize"));
                res.window_system_data_split = Some(ev.split);
                res.data_float = Some(HashableF32::new(ev.ratio));
                Message::DomEvent(-1, res)
            });

        // Register any events here
        for event in events {
            match event.event_type.as_str() {
                "click" => {
                    pane_grid = pane_grid.on_click(move |pane| {
                        let mut res = EventResponse::new(me, String::from("click"));
                        let internal_pane = self.panes.get(pane).unwrap();
                        let pane_id = match internal_pane {
                            InternalPane(pane_id, _) => pane_id.clone(),
                        };
                        res.data_str = Some(pane_id);
                        res.window_system_data_window = Some(pane);
                        Message::DomEvent(event.event_uid, res)
                    });
                }
                "drag" => {
                    pane_grid = pane_grid.on_drag(move |ev| {
                        Message::DomEvent(event.event_uid, self.preprocess_pane_drag(ev, me))
                    })
                }
                "resize" => {
                    pane_grid = pane_grid.on_resize(self.drag_border_size, move |ev| {
                        let mut res = EventResponse::new(me, String::from("resize"));
                        res.window_system_data_split = Some(ev.split);
                        res.data_float = Some(HashableF32::new(ev.ratio));
                        Message::DomEvent(event.event_uid, res)
                    })
                }
                _ => (),
            }
        }

        return pane_grid.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let hintmsg = "WindowSystem focus event must include a window ID. Hint: use DomEvent::new().with(\"window\", window_id) to include the window ID.";
        match event {
            XmlChangeEvent::EmittedEvent(name, datas) => match name.as_str() {
                "focus" => {
                    if datas.datas_str.value().get("window").is_none() {
                        panic!("{}", hintmsg);
                    }
                    self.focused = self
                        .window_id_to_pane(
                            datas
                                .datas_str
                                .value()
                                .get("window")
                                .as_ref()
                                .unwrap()
                                .as_str(),
                        )
                        .unwrap();
                    None
                }
                "fullscreen" => {
                    if datas.datas_str.value().get("window").is_none() {
                        panic!("{}", hintmsg);
                    }
                    let pane = self
                        .window_id_to_pane(
                            datas
                                .datas_str
                                .value()
                                .get("window")
                                .as_ref()
                                .unwrap()
                                .as_str(),
                        )
                        .unwrap();
                    self.panes.maximize(pane);
                    None
                }
                "restore" => {
                    self.panes.restore();
                    None
                }
                "close" => {
                    if datas.datas_str.value().get("window").is_none() {
                        panic!("{}", hintmsg);
                    }
                    let pane = self
                        .window_id_to_pane(
                            datas
                                .datas_str
                                .value()
                                .get("window")
                                .as_ref()
                                .unwrap()
                                .as_str(),
                        )
                        .unwrap();
                    let datas_op = self.panes.close(pane);
                    if datas_op.is_some() {
                        let (_, to_focus) = datas_op.unwrap();
                        self.focused = to_focus;
                    }
                    None
                }
                "open" => {
                    if datas.datas_str.value().get("window").is_none() {
                        panic!("{}", hintmsg);
                    }
                    let window_id = datas
                        .datas_str
                        .value()
                        .get("window")
                        .as_ref()
                        .unwrap()
                        .as_str();
                    if self.children_id.get(window_id).is_none() {
                        panic!(
                            "WindowSystem open event: window-id {} not found.",
                            window_id
                        );
                    }
                    let child_uid = self.children_id.get(window_id).unwrap();
                    let datas_attributes = datas.datas_str.value();
                    let parent_pane_id = datas_attributes.get("parent");
                    if parent_pane_id.is_none() {
                        panic!(
                            "WindowSystem focus event must include a parent. Hint: use DomEvent::new().with(\"parent\", parent_window_id) to include the parent."
                        );
                    }
                    let parent_pane = self.window_id_to_pane(parent_pane_id.unwrap());
                    if parent_pane.is_none() {
                        println!(
                            "WindowSystem open event: parent window-id {} not found.",
                            parent_pane_id.unwrap()
                        );
                        return None;
                    }
                    let split_method = datas_attributes.get("split-method");
                    if self.window_id_to_pane(window_id).is_some() {
                        println!(
                            "Window with id: {} is already opened. Not opening a new one.",
                            window_id
                        );
                        return None;
                    }
                    let pane = self.panes.split(
                        parse_pane_axis(split_method.unwrap().as_str()),
                        parent_pane.unwrap(),
                        InternalPane(window_id.to_string(), child_uid.clone()),
                    );
                    if pane.is_some() {
                        self.focused = pane.unwrap().0;
                    }
                    None
                }
                _ => None,
            },
            XmlChangeEvent::EventFired(t, response) => {
                if t == "maximize" {
                    self.panes
                        .maximize(response.window_system_data_window.unwrap());
                }
                if t == "restore" {
                    self.panes.restore();
                }
                if t == "close" {
                    let datas_op = self
                        .panes
                        .close(response.window_system_data_window.unwrap());
                    if datas_op.is_some() {
                        let (_, to_focus) = datas_op.unwrap();
                        self.focused = to_focus;
                    }
                }
                if t == "click" {
                    self.focused = response.window_system_data_window.unwrap();
                }
                if t == "resize" {
                    self.panes.resize(
                        response.window_system_data_split.unwrap(),
                        response.data_float.as_ref().unwrap().value(),
                    );
                }
                if t == "drag" {
                    let event_type = response.data_str.as_ref().unwrap();
                    match event_type.as_str() {
                        "dropped" => {
                            self.panes.drop(
                                response.window_system_data_window.unwrap(),
                                response.window_system_data_target.as_ref().unwrap().value(),
                            );
                        }
                        _ => {}
                    }
                }
                return None;
            }
            _ => None,
        }
    }
}
