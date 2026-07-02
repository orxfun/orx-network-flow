use crate::graphs::VecEdge;
use good_lp::Variable;

#[cfg(feature = "solver-lp-solvers")]
use good_lp::Solver;

#[cfg(feature = "solver-lp-solvers")]
pub unsafe fn lp_solvers_model_to_problem<S: Solver>(
    model: &S::Model,
) -> &lp_solvers::problem::Problem {
    let x = model as *const S::Model as *const lp_solvers::problem::Problem;
    unsafe { &*x }
}

#[cfg(all(feature = "solver-lp-solvers", feature = "std"))]
pub unsafe fn lp_solvers_model_to_lp_file<S: Solver, P>(
    model: &S::Model,
    path: P,
) -> Result<(), std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    use lp_solvers::lp_format::LpProblem;
    use std::io::Write;

    let p = unsafe { lp_solvers_model_to_problem::<S>(model) };

    let f = std::fs::File::create(path)?;
    let mut f = std::io::BufWriter::new(f);
    write!(f, "{}", p.display_lp())?;
    f.flush()?;

    Ok(())
}

#[cfg(feature = "solver-lp-solvers")]
pub struct FlowsByEdges {
    pub solution: good_lp::solvers::lp_solvers::LpSolution,
    pub vars: VecEdge<Variable>,
}
