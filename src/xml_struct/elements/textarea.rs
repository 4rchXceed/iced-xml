use iced::{Point, widget::text_editor::Motion::*};

// Copy-paste template
use crate::{
    dom::{
        events::DomInternalMessageType,
        query_builder::{CustomElementEvent, EventResponse, QueryResponse},
    },
    rs_utils::{HashableF32, HashableTextareaEdit, Vector2},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::element_base::ElementBase,
        parser::XmlElement,
    },
};

#[derive(Debug, Clone, Hash)]
pub enum TextareaEvent {
    Click(Vector2),
    Drag(Vector2),
    Input((HashableTextareaEdit, String)),
    CursorMove(String),
    Scroll(i32),
    MoveSelection(String),
    SelectAll,
    SelectLine,
    SelectWord,
}

pub struct Textarea {
    content: iced::widget::text_editor::Content,
    placeholder: String,
}

fn handle_event(content: &mut iced::widget::text_editor::Content, event: &TextareaEvent) {
    let action = match event {
        TextareaEvent::Click(p) => iced::widget::text_editor::Action::Click(Point {
            x: p.x.value(),
            y: p.y.value(),
        }),
        TextareaEvent::Drag(p) => iced::widget::text_editor::Action::Drag(Point {
            x: p.x.value(),
            y: p.y.value(),
        }),
        TextareaEvent::Input(e) => iced::widget::text_editor::Action::Edit(e.0.value().clone()),
        TextareaEvent::MoveSelection(m) => {
            let motion = generate_motion(&m).unwrap_or(iced::widget::text_editor::Motion::Down);
            iced::widget::text_editor::Action::Select(motion)
        }
        TextareaEvent::Scroll(lines) => iced::widget::text_editor::Action::Scroll { lines: *lines },
        TextareaEvent::SelectAll => iced::widget::text_editor::Action::SelectAll,
        TextareaEvent::SelectLine => iced::widget::text_editor::Action::SelectLine,
        TextareaEvent::SelectWord => iced::widget::text_editor::Action::SelectWord,
        TextareaEvent::CursorMove(m) => {
            let motion = generate_motion(&m).unwrap_or(iced::widget::text_editor::Motion::Down);
            iced::widget::text_editor::Action::Move(motion)
        }
    };
    content.perform(action);
}

fn parse_motion(motion: iced::widget::text_editor::Motion) -> String {
    return String::from(match motion {
        DocumentStart => "DocumentStart",
        DocumentEnd => "DocumentEnd",
        Down => "Down",
        Home => "Home",
        Left => "Left",
        PageDown => "PageDown",
        PageUp => "PageUp",
        Right => "Right",
        WordLeft => "WordLeft",
        WordRight => "WordRight",
        Up => "Up",
        End => "End",
    });
}

fn generate_motion(motion_str: &str) -> Option<iced::widget::text_editor::Motion> {
    return match motion_str {
        "DocumentStart" => Some(DocumentStart),
        "DocumentEnd" => Some(DocumentEnd),
        "Down" => Some(Down),
        "Home" => Some(Home),
        "Left" => Some(Left),
        "PageDown" => Some(PageDown),
        "PageUp" => Some(PageUp),
        "Right" => Some(Right),
        "WordLeft" => Some(WordLeft),
        "WordRight" => Some(WordRight),
        "Up" => Some(Up),
        "End" => Some(End),
        _ => None,
    };
}

fn generate_action(
    action: iced::widget::text_editor::Action,
    content: &iced::widget::text_editor::Content,
) -> TextareaEvent {
    return match action {
        iced::widget::text_editor::Action::Click(Point { x, y }) => TextareaEvent::Click(Vector2 {
            x: HashableF32::new(x),
            y: HashableF32::new(y),
        }),
        iced::widget::text_editor::Action::Drag(Point { x, y }) => TextareaEvent::Drag(Vector2 {
            x: HashableF32::new(x),
            y: HashableF32::new(y),
        }),
        iced::widget::text_editor::Action::Edit(edit) => {
            TextareaEvent::Input((HashableTextareaEdit::new(edit), content.text()))
        }
        iced::widget::text_editor::Action::Move(motion) => {
            TextareaEvent::CursorMove(parse_motion(motion))
        }
        iced::widget::text_editor::Action::Scroll { lines } => TextareaEvent::Scroll(lines),
        iced::widget::text_editor::Action::Select(motion) => {
            TextareaEvent::MoveSelection(parse_motion(motion))
        }
        iced::widget::text_editor::Action::SelectAll => TextareaEvent::SelectAll,
        iced::widget::text_editor::Action::SelectLine => TextareaEvent::SelectLine,
        iced::widget::text_editor::Action::SelectWord => TextareaEvent::SelectWord,
    };
}

impl ElementBase for Textarea {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        let content = iced::widget::text_editor::Content::with_text(&xml_element.text);
        let mut placeholder = String::new();

        if xml_element.attributes.contains_key("placeholder") {
            placeholder = xml_element.attributes["placeholder"].clone();
        }

        Self {
            content: content,
            placeholder: placeholder,
        }
    }

    fn render<'a>(
        &'a self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();

        let mut textarea = iced::widget::text_editor::TextEditor::new(&self.content);

        textarea = textarea
            .placeholder(&self.placeholder)
            .font(theme.font)
            .height(theme.height)
            .line_height(theme.line_height)
            .max_height(theme.max_height)
            .min_height(theme.textarea_min_height)
            .padding(theme.padding)
            .wrapping(theme.text_wrapping)
            .style(move |_, _| iced::widget::text_editor::Style {
                background: theme.background,
                border: iced::Border {
                    color: theme.border_color,
                    width: theme.border_width,
                    radius: theme.border_radius,
                },
                placeholder: theme.input_placeholder_color,
                value: theme.foreground_color,
                selection: theme.selection_color,
            }); // highlight_with??
        if theme.size.is_some() {
            textarea = textarea.size(theme.size.unwrap());
        }
        if theme.textarea_width.is_some() {
            textarea = textarea.width(theme.textarea_width.unwrap());
        }

        // TODO (well maybe): Key mapping

        // Events
        let mut event_uid = None;

        for event in events {
            if event.event_type == "textarea_event" {
                event_uid = Some(event.event_uid);
                break;
            }
        }

        let me = self_uid;
        textarea = textarea.on_action(move |action| {
            let mut ev_response = EventResponse::new(me, String::from("textarea_event"));
            ev_response.textarea_event = Some(generate_action(action, &self.content));
            return Message::DomEvent(event_uid, ev_response);
        });

        return textarea.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::FireEvent(custom_event) => match custom_event {
                CustomElementEvent::TextareaEvent(textarea_event) => {
                    handle_event(&mut self.content, textarea_event);
                    return Some(ElementEventResponse::success());
                }
                CustomElementEvent::MoveCursor(to) => {
                    self.content.move_to(iced::widget::text_editor::Cursor {
                        position: iced::widget::text_editor::Position {
                            line: to.x.value() as usize,
                            column: to.y.value() as usize,
                        },
                        selection: None,
                    });
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::PropertyChange(name, new_val) => match name.as_str() {
                "placeholder" => {
                    self.placeholder = new_val.clone();
                    return Some(ElementEventResponse::success());
                }
                "value" => {
                    self.content = iced::widget::text_editor::Content::with_text(new_val);
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(name) => match name.as_str() {
                "placeholder" => {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_str(self.placeholder.clone()),
                    ));
                }
                "value" => {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_str(self.content.text().to_string()),
                    ));
                }
                "cursor_position" => {
                    let pos = self.content.cursor().position;
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_vector(Vector2 {
                            x: HashableF32::new(pos.column as f32),
                            y: HashableF32::new(pos.line as f32),
                        }),
                    ));
                }
                "selection" => {
                    let selection = self.content.selection();
                    if selection.is_some() {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_str(selection.unwrap()),
                        ));
                    } else {
                        return Some(ElementEventResponse::success());
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &String,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type.as_str() {
            "textarea_event" => {
                if event_response.textarea_event.is_some() {
                    handle_event(
                        &mut self.content,
                        event_response.textarea_event.as_ref().unwrap(),
                    );
                    return Some(ElementEventResponse::success());
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }
}
