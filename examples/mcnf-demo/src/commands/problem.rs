use crate::problem_builder::build_problem_from_input;
use crate::serialization::{McnfResponse, NetworkChoice, ProblemInput};
use crate::solver_handler::solve_network;
use crate::{AppState, StoredProblemData};
use std::sync::Arc;

/// Build a problem from frontend input and store it
pub async fn build_problem(input: ProblemInput, state: Arc<AppState>) -> Result<String, String> {
    // Validate the input
    let _problem = build_problem_from_input(input.clone())?;

    // Store the input data
    let mut current = state.current_problem.lock().unwrap();
    *current = Some(StoredProblemData { input });

    Ok("Problem built successfully".into())
}

/// Get statistics for the current problem with network configuration
pub async fn get_problem_stats(
    network_choice: NetworkChoice,
    state: Arc<AppState>,
) -> Result<McnfResponse, String> {
    let current = state.current_problem.lock().unwrap();
    let data = current.as_ref().ok_or("No problem loaded")?;

    // Rebuild problem from stored input
    let problem = build_problem_from_input(data.input.clone())?;

    solve_network(&problem, &network_choice)
}
