use std::any::Any;

use iced::{Element, Renderer, Subscription, Task, Theme, application, daemon::ViewFn, window};

use crate::{app_manager::state_wrapper::App, window_wrapper::AppResult, xml_engine::Message};

// Expose the types for the macro
pub type IcedElement<'a> = Element<'a, Message>;
pub type IcedSubscription = Subscription<Message>;
pub type IcedTask = iced::Task<Message>;
pub type WindowId = window::Id;
pub type WindowSettings = window::Settings;

// Expose the FNs for the macro
pub fn open_window(settings: WindowSettings) -> (WindowId, Task<WindowId>) {
    return window::open(settings);
}

pub fn close_event_messages() -> Subscription<Message> {
    return window::close_events().map(|id| Message::WindowClosed(id));
}

pub fn exit_iced() -> Task<Message> {
    return iced::exit();
}

pub fn close_window(id: WindowId) -> Task<Message> {
    return window::close(id);
}

/// ComponentFunctions is a struct that holds the functions for rendering, updating, subscribing and handling window close events
#[derive(Debug, Clone)]
pub struct ComponentFunctions {
    pub render: fn(&Box<dyn Any>) -> IcedElement<'_>,
    pub update: fn(&mut Box<dyn Any>, Message, &mut dyn Any),
    pub subscribe: fn(&Box<dyn Any>) -> Vec<Subscription<Message>>,
    pub on_close: fn(&mut Box<dyn Any>, &mut dyn Any),
}

/// Run the application with the given boot, update, view and subscribe functions
pub fn run_app_internal<State: 'static>(
    boot: impl application::BootFn<State, Message> + 'static,
    update: impl application::UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
    subscribe: impl Fn(&State) -> Subscription<Message> + 'static,
) -> AppResult {
    return iced::daemon(boot, update, view)
        .subscription(subscribe)
        .run();
}

/// Create a text element with the given text
pub fn text(text: &str) -> IcedElement<'_> {
    return iced::widget::text(text).into();
}

pub type CreateWindowFn<W, C, S> = fn(params: C, app_state: &mut App<S>, window_id: WindowId) -> W;

pub fn error(e: &str) {
    log::error!("Error: {}", e);
}
