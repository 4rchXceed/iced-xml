use std::any::Any;

use iced::{Element, Renderer, Subscription, Task, Theme, application, daemon::ViewFn, window};

use crate::{window_wrapper::AppResult, xml_engine::Message};

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

#[derive(Debug, Clone)]
pub struct ComponentFunctions {
    pub render: fn(&Box<dyn Any>) -> IcedElement<'_>,
    pub update: fn(&mut Box<dyn Any>, Message, &mut dyn Any),
    pub subscribe: fn(&Box<dyn Any>) -> Vec<Subscription<Message>>,
    pub on_close: fn(&mut Box<dyn Any>, &mut dyn Any),
}

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

pub fn text(text: &str) -> IcedElement<'_> {
    return iced::widget::text(text).into();
}

pub struct App<State> {
    pub state: State,
    window_params: Vec<Box<dyn Any>>, // This is used to store the creation parameters for each window
    window_closure_queue: Vec<WindowId>, // This is used to store the IDs of windows that need to be closed
}

impl<State> App<State> {
    pub fn new(state: State) -> Self {
        return Self {
            state: state,
            window_params: Vec::new(),
            window_closure_queue: Vec::new(),
        };
    }

    pub fn open_window(&mut self, params: Box<dyn Any>) {
        self.window_params.push(params);
    }

    pub fn close_window(&mut self, id: WindowId) {
        self.window_closure_queue.push(id);
    }

    pub fn get_window_params(&mut self) -> Vec<Box<dyn Any>> {
        let mut window_params = Vec::new();
        for params in self.window_params.drain(..) {
            window_params.push(params);
        }
        return window_params;
    }

    pub fn get_window_closure_queue(&mut self) -> Vec<WindowId> {
        let mut window_closure_queue = Vec::new();
        for id in self.window_closure_queue.drain(..) {
            window_closure_queue.push(id);
        }
        return window_closure_queue;
    }

    pub fn into_any<'a>(&'a mut self) -> Box<dyn Any + 'a> {
        return Box::new(self);
    }
}

#[macro_export]
macro_rules! window_manager {
    (
            $windows:ident { $( $variant:ident ),* $(,)? };
            $create_window:path;
            $state:ty;
            $window_creation_params:ty
        ) => {
        use iced_xml::dom::query::EventResponse;
        use iced_xml::app_manager::{
            IcedSubscription,
            IcedElement,
            IcedTask,
            WindowSettings,
            WindowId,
            open_window,
            text,
            run_app_internal,
            App,
            close_event_messages,
            exit_iced,
            close_window,
            ComponentFunctions
        };


        use std::any::Any;
        use std::collections::BTreeMap;
        use std::collections::HashMap;

        use iced_xml::rs_utils::get_unique_id;
        use iced_xml::xml_engine::Message;

        pub enum $windows {
            $( $variant($variant) ),*
        }

        pub fn render_component(window: &Box<dyn Any>) -> IcedElement<'_> {
            if !window.is::<$windows>() {
                panic!("Invalid window type. When you call add_component(i32, Box<dyn Any>) set the second argument to the Windows *ENUM* variant, not the struct itself. Example: add_component(1, Windows::MyWindow(...))");
            }
            let window = window.downcast_ref::<$windows>().unwrap();
            return match window {
                $(
                    $windows::$variant(w) => w.render(),
                )*
            };
        }

        pub fn update_component(window: &mut Box<dyn Any>, message: Message, state: &mut dyn Any) {
            if !window.is::<$windows>() {
                panic!("Invalid window type");
            }
            if !state.is::<App<$state>>() {
                panic!("Invalid state type");
            }

            let window = window.downcast_mut::<$windows>().unwrap();
            let state = state.downcast_mut::<App<$state>>().unwrap();
            match window {
                $(
                    $windows::$variant(w) => w.update(message, state),
                )*
            };
        }

        pub fn subscribe_component(window: &Box<dyn Any>) -> Vec<IcedSubscription> {
            if !window.is::<$windows>() {
                panic!("Invalid window type");
            }
            let window = window.downcast_ref::<$windows>().unwrap();
            return match window {
                $(
                    $windows::$variant(w) => w.subscription(),
                )*
            };
        }

        pub fn on_close_component(window: &mut Box<dyn Any>, state: &mut dyn Any) {
            if !window.is::<$windows>() {
                panic!("Invalid window type");
            }
            if !state.is::<App<$state>>() {
                panic!("Invalid state type");
            }
            let window = window.downcast_mut::<$windows>().unwrap();
            let state = state.downcast_mut::<App<$state>>().unwrap();
            match window {
                $(
                    $windows::$variant(w) => w.on_close(state),
                )*
            };
        }

        pub static DEFAULT_ENGINE_SETTINGS: iced_xml::xml_engine::EngineSettings = iced_xml::xml_engine::EngineSettings {
            fonts: iced_xml::xml_struct::theming::Fonts::new(),
            functions: ComponentFunctions {
                render: render_component,
                update: update_component,
                subscribe: subscribe_component,
                on_close: on_close_component,
            }
        };

        struct WindowManager {
            windows: BTreeMap<WindowId, $windows>,
            creation_params: HashMap<i32, $window_creation_params>, // This is used to store the creation parameters for each window
            app_state: App<$state>, // This is used to store the state of the application
        }

        impl WindowManager {
            fn new(main_window_params: $window_creation_params, state: $state) -> (Self, IcedTask) {
                let (_, open) = open_window(WindowSettings::default());
                let mut creation_params = HashMap::new();
                creation_params.insert(-1, main_window_params);
                return (
                    Self {
                        windows: BTreeMap::new(),
                        creation_params: creation_params,
                        app_state: App::new(state),
                    },
                    open.map(|id| Message::WindowOpened(id, -1)),
                );
            }

            fn update(&mut self, message: Message) -> IcedTask {
                let mut tasks = Vec::new();
                for params in self.app_state.get_window_params() {
                    if !params.is::<$window_creation_params>() {
                        panic!("Invalid window creation params type");
                    }
                    let params = *params.downcast::<$window_creation_params>().unwrap();
                    let params_id = get_unique_id();
                    self.creation_params.insert(params_id, params);
                    tasks.push(IcedTask::perform(async move { params_id }, |params_id| {
                        Message::OpenWindow(params_id)
                    }));
                }
                for id in self.app_state.get_window_closure_queue() {
                    tasks.push(close_window(id));
                }
                let msg_task = match message {
                    Message::WindowOpened(id, params_id) => {
                        if !self.creation_params.contains_key(&params_id) {
                            panic!("Window creation params not found for id: {}", params_id);
                        }
                        let window =
                            create_window(self.creation_params.get(&params_id).unwrap().clone(), &mut self.app_state, id);
                        self.windows.insert(id, window);
                        IcedTask::none()
                    }
                    Message::WindowClosed(id) => {
                        let mut window = self.windows.remove(&id);
                        match window {
                            Some(window) => match window {
                                $(
                                    $windows::$variant(mut w) => w.on_close(&mut self.app_state),
                                )*
                            },
                            None => {}
                        }
                        if self.windows.is_empty() {
                            exit_iced()
                        } else {
                            IcedTask::none()
                        }
                    }
                    Message::OpenWindow(params_id) => {
                        if !self.creation_params.contains_key(&params_id) {
                            panic!("Window creation params not found for id: {}", params_id);
                        }
                        let (_, open) = open_window(WindowSettings::default());
                        open.map(move |id| Message::WindowOpened(id, params_id))
                    }
                    _ => {
                        for (_, window) in self.windows.iter_mut() {
                            match window {
                                $(
                                    $windows::$variant(w) => w.update(message.clone(), &mut self.app_state),
                                )*
                            };
                        }
                        IcedTask::none()
                    }
                };
                tasks.push(msg_task);
                return IcedTask::batch(tasks);
            }

            fn view(&self, window_id: WindowId) -> IcedElement<'_> {
                match self.windows.get(&window_id) {
                    Some(window) => match window {
                        $(
                            $windows::$variant(w) => w.render(),
                        )*
                    },
                    None => {
                        return text("Window not found").into();
                    }
                }
            }

            fn subscription(&self) -> IcedSubscription {
                let mut subs = Vec::new();
                subs.push(close_event_messages());
                for (_, window) in self.windows.iter() {
                    match window {
                        $(
                            $windows::$variant(w) => {
                                subs.append(&mut w.subscription());
                            },
                        )*
                    }
                }
                return IcedSubscription::batch(subs);
            }
        }

        pub fn run_app(main_window_params: $window_creation_params, gen_state: impl Fn() -> $state + 'static) -> AppResult {
            return run_app_internal(
                move || {
                    let app = WindowManager::new(main_window_params.clone(), gen_state());
                    return app;
                },
                WindowManager::update,
                WindowManager::view,
                WindowManager::subscription
            );
        }
    };
}
