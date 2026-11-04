use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex, mpsc::Receiver},
    time::Duration,
};

use iced::time;
use log::error;

use crate::{
    app_manager::{
        state_wrapper::App,
        utils::{
            CloseMode, CreateWindowFn, IcedElement, IcedSubscription, IcedTask, WindowId,
            WindowSettings, close_event_messages, close_window, exit_iced, open_window,
            run_app_internal, text,
        },
    },
    rs_utils::{IntoAny, get_unique_id},
    window_wrapper::AppResult,
    xml_engine::{EngineSettings, Message},
};

pub struct WindowManager<W, C, S>
where
    W: 'static + Any + IntoAny,
    C: 'static + Clone + Any,
    S: 'static + Any,
{
    windows: BTreeMap<WindowId, Box<dyn Any>>,
    creation_params: HashMap<i32, C>, // This is used to store the creation parameters for each window
    app_state: App<S>,                // This is used to store the state of the application
    engine_params: EngineSettings,
    create_window: CreateWindowFn<W, C, S>,
    close_mode: CloseMode,
    open_sender: Arc<Mutex<Receiver<C>>>,
}

impl<W, C, S> WindowManager<W, C, S>
where
    W: 'static + Any + IntoAny,
    C: 'static + Clone + Any,
    S: 'static + Any,
{
    fn new(
        main_window_params: C,
        state: S,
        engine_settings: EngineSettings,
        create_window: CreateWindowFn<W, C, S>,
        close_mode: CloseMode,
        open_window_sender: Arc<Mutex<Receiver<C>>>,
    ) -> (Self, IcedTask) {
        let (_, open) = open_window(WindowSettings::default());

        let mut creation_params = HashMap::new();

        creation_params.insert(-1, main_window_params);

        return (
            Self {
                windows: BTreeMap::new(),
                creation_params: creation_params,
                app_state: App::new(state),
                engine_params: engine_settings,
                create_window: create_window,
                close_mode: close_mode,
                open_sender: open_window_sender,
            },
            open.map(|id| Message::WindowOpened(id, -1)),
        );
    }

    fn update(&mut self, message: Message) -> IcedTask {
        let mut tasks = Vec::new();

        for params in self.app_state.get_window_params() {
            // if !params.is::<C>() {
            // println!("Invalid window creation params type");
            if let Ok(params) = params.downcast::<C>() {
                let params_id = get_unique_id();

                self.creation_params.insert(params_id, *params);

                tasks.push(IcedTask::perform(async move { params_id }, |params_id| {
                    Message::OpenWindow(params_id)
                }));
            } else {
                error!("Invalid window creation params type");
            }
        }

        for id in self.app_state.get_window_closure_queue() {
            tasks.push(close_window(id));
        }

        let msg_task = self.handle_message(message);

        tasks.push(msg_task);

        return IcedTask::batch(tasks);
    }

    fn handle_message(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::TasksUpdate => {
                if let Ok(window_params) = self.open_sender.lock() {
                    let mut tasks = Vec::new();

                    while let Ok(params) = window_params.try_recv() {
                        let (_, open) = open_window(WindowSettings::default());

                        let params_id = get_unique_id();

                        self.creation_params.insert(params_id, params);

                        let current_task = open.map(move |id| Message::WindowOpened(id, params_id));

                        tasks.push(current_task);
                    }

                    IcedTask::batch(tasks)
                } else {
                    IcedTask::none()
                }
            }
            Message::WindowOpened(id, params_id) => {
                if let Some(creation_params) = self.creation_params.get(&params_id) {
                    let create_window = self.create_window;

                    let window = create_window(creation_params.clone(), &mut self.app_state, id);

                    self.windows.insert(id, window.into_any());
                } else {
                    error!("Window creation params not found for id: {}", params_id);
                }
                IcedTask::none()
            }
            Message::WindowClosed(id) => {
                let window = self.windows.remove(&id);

                match window {
                    Some(window) => {
                        let on_close = self.engine_params.functions.on_close;

                        on_close(&mut Box::new(window), &mut self.app_state);
                    }
                    None => {}
                }

                if self.windows.is_empty() {
                    match self.close_mode {
                        CloseMode::Close => exit_iced(),
                        CloseMode::Minimize => IcedTask::none(),
                    }
                } else {
                    IcedTask::none()
                }
            }
            Message::OpenWindow(params_id) => {
                if !self.creation_params.contains_key(&params_id) {
                    error!("Window creation params not found for id: {}", params_id);

                    IcedTask::none()
                } else {
                    let (_, open) = open_window(WindowSettings::default());

                    open.map(move |id| Message::WindowOpened(id, params_id))
                }
            }
            _ => {
                for (_, window) in self.windows.iter_mut() {
                    let update = self.engine_params.functions.update;

                    update(&mut *window, message.clone(), &mut self.app_state);
                }
                IcedTask::none()
            }
        }
    }

    fn view(&self, window_id: WindowId) -> IcedElement<'_> {
        let render = self.engine_params.functions.render;

        match self.windows.get(&window_id) {
            Some(window) => render(&*window),
            None => text("Window not found").into(),
        }
    }

    fn subscription(&self) -> IcedSubscription {
        let mut subs = Vec::new();

        subs.push(close_event_messages());

        let subscribe = self.engine_params.functions.subscribe;

        for (_, window) in self.windows.iter() {
            let window_subs = subscribe(&*window);

            for sub in window_subs {
                subs.push(sub);
            }
        }

        subs.push(
            time::every(Duration::from_millis(100))
                .with(Message::TasksUpdate)
                .map(|a| a.0),
        );

        return IcedSubscription::batch(subs);
    }

    pub fn run_app(
        main_window_params: C,
        gen_state: impl Fn() -> S + 'static,
        engine_settings: EngineSettings,
        create_window: CreateWindowFn<W, C, S>,
        close_mode: CloseMode,
        window_opener: Receiver<C>,
    ) -> AppResult {
        let window = Arc::new(Mutex::new(window_opener));

        return run_app_internal(
            move || {
                let app = WindowManager::new(
                    main_window_params.clone(),
                    gen_state(),
                    engine_settings.clone(),
                    create_window,
                    close_mode.clone(),
                    window.clone(),
                );

                return app;
            },
            WindowManager::update,
            WindowManager::view,
            WindowManager::subscription,
        );
    }
}

/// The window_manager macro is the mess of code that generates the nessary structure to manage multiple windows.
/// !! This will be removed in the V0.1.0 !!
///
/// TODO: Doc + examples
#[macro_export]
macro_rules! window_manager {
    (
            $windows:ident { $( $variant:ident ),* $(,)? };
            $state:ty;
            $window_creation_params:ty
        ) => {
        pub enum $windows {
            $( $variant($variant) ),*
        }

        impl iced_xml::rs_utils::IntoAny for $windows {
            fn into_any(self) -> Box<dyn std::any::Any> {
                Box::new(self)
            }
        }

        pub fn render_component(window: &Box<dyn std::any::Any>) -> iced_xml::app_manager::utils::IcedElement<'_> {
            use iced_xml::window_wrapper::WindowTemplate;
            if let Some(window) = window.downcast_ref::<$windows>() {
                return match window {
                    $(
                        $windows::$variant(w) => w.render(),
                    )*
                };
            } else {
                return iced_xml::app_manager::utils::text("Invalid window type. When you call add_component(i32, Box<dyn Any>) set the second argument to the Windows *ENUM* variant, not the struct itself. Example: add_component(1, Windows::MyWindow(...))").into();
            }
        }

        pub fn update_component(window: &mut Box<dyn std::any::Any>, message: iced_xml::xml_engine::Message, state: &mut dyn std::any::Any) {
            use iced_xml::window_wrapper::WindowTemplate;
            if let Some(window) = window.downcast_mut::<$windows>() && let Some(state) = state.downcast_mut::<iced_xml::app_manager::state_wrapper::App<$state>>() {
                match window {
                    $(
                        $windows::$variant(w) => w.update(message, state),
                    )*
                };
            } else {
                iced_xml::app_manager::utils::error("Invalid state/window type");
            }
        }

        pub fn subscribe_component(window: &Box<dyn std::any::Any>) -> Vec<iced_xml::app_manager::utils::IcedSubscription> {
            use iced_xml::window_wrapper::WindowTemplate;
            if let Some(window) = window.downcast_ref::<$windows>() {
                return match window {
                    $(
                        $windows::$variant(w) => w.subscription(),
                    )*
                };
            } else {
                iced_xml::app_manager::utils::error("Invalid window type");
                return Vec::new();
            }
        }

        pub fn on_close_component(window: &mut Box<dyn std::any::Any>, state: &mut dyn std::any::Any) {
            use iced_xml::window_wrapper::WindowTemplate;
            if let Some(window) = window.downcast_mut::<$windows>() && let Some(state) = state.downcast_mut::<iced_xml::app_manager::state_wrapper::App<$state>>() {
                match window {
                    $(
                        $windows::$variant(w) => w.on_close(state),
                    )*
                };
            } else {
                iced_xml::app_manager::utils::error("Invalid state/window type");
            }
        }

        pub static DEFAULT_ENGINE_SETTINGS: iced_xml::xml_engine::EngineSettings = iced_xml::xml_engine::EngineSettings {
            fonts: iced_xml::xml_struct::theming::Fonts::new(),
            functions: iced_xml::app_manager::utils::ComponentFunctions {
                render: render_component,
                update: update_component,
                subscribe: subscribe_component,
                on_close: on_close_component,
            }
        };
    };
}
