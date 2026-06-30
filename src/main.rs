use iced::application::IntoBoot;

use crate::{app::app::UserApp, xml_engine::XmlEngine};

pub mod app;
pub mod dom;
pub mod logger;
pub mod utilsfn;
pub mod xml_engine;
pub mod xml_struct;

fn init() -> UserApp {
    let mut app = UserApp::new();
    app.init();
    return app;
}

pub fn main() -> iced::Result {
    return iced::application(init, UserApp::update, UserApp::render).run();
}
