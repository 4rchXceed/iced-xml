use std::any::Any;

use crate::app_manager::utils::WindowId;

/// App is a wrapper for the app's State with a few utility functions
pub struct App<State> {
    pub state: State,
    window_params: Vec<Box<dyn Any>>, // This is used to store the creation parameters for each window
    window_closure_queue: Vec<WindowId>, // This is used to store the IDs of windows that need to be closed
}

impl<State> App<State> {
    /// Create a new App with the given state
    pub fn new(state: State) -> Self {
        return Self {
            state: state,
            window_params: Vec::new(),
            window_closure_queue: Vec::new(),
        };
    }

    /// Opens a new window with the specified parameters.
    pub fn open_window(&mut self, params: Box<dyn Any>) {
        self.window_params.push(params);
    }

    /// Closes the window with the specified ID.
    pub fn close_window(&mut self, id: WindowId) {
        self.window_closure_queue.push(id);
    }

    /// Gets all of the window params
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
