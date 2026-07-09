use std::sync::Mutex;

pub mod app;
pub mod commands;
pub mod components;
pub mod problem_builder;
pub mod serialization;
pub mod solver_handler;

pub use app::App;

// Store problem input data instead of Problem struct
#[derive(Clone)]
pub struct StoredProblemData {
    pub input: serialization::ProblemInput,
}

// Application state
pub struct AppState {
    pub current_problem: Mutex<Option<StoredProblemData>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_problem: Mutex::new(None),
        }
    }
}
