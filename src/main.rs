mod utilsfn;
mod logger;
mod xml_struct;
use std::io::Cursor;

use quick_xml::{Reader};
use utilsfn::safe_read_file;
use xml_struct::window::XmlWindow;

use crate::xml_struct::{parser::XmlParser};

const MAIN_WINDOW: &str = "src/main.xml";

#[derive(Debug, Clone)]
enum Message {
}

struct App {
    window: XmlWindow,
}

impl App {
    fn new() -> Self {
        let content: String = safe_read_file(MAIN_WINDOW);
        let reader = Reader::from_reader(Cursor::new(content.into_bytes()));
        let window_parser = XmlParser::new(&mut reader.clone());
        let window = XmlWindow::new(window_parser.root);
        Self {
            window,
        }
    }
    fn update(&mut self, message: Message) {
        match message {
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        return self.window.render();
    }
}

pub fn main() -> iced::Result {
    return iced::application(App::new, App::update, App::view).run();
}
