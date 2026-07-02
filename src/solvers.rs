//! Convenience re-exports for LP/MILP solver constructors.
//!
//! Enable the corresponding Cargo feature to use each solver:
//!
//! | Feature | Solver | External requirement |
//! |---|---|---|
//! | `solver-microlp` *(default)* | microlp | none — pure Rust |
//! | `solver-highs` | HiGHS | C++ compiler + cmake at build time |
//! | `solver-scip` | SCIP | local SCIP install |
//! | `solver-scip-bundled` | SCIP | none — ships a bundled binary |
//! | `solver-cbc` | CBC | `coinor-cbc` system library |
//! | `solver-lp-solvers` | external binary (CPLEX, Gurobi …) | solver binary at runtime |
//! | `solver-cplex-rs` | CPLEX (static) | local CPLEX install + clang |

/// The [microlp](https://docs.rs/microlp) pure-Rust solver.
/// Available with feature `solver-microlp` (enabled by default).
#[cfg(feature = "solver-microlp")]
pub use good_lp::solvers::microlp::microlp;

/// The [HiGHS](https://highs.dev/) parallel MILP solver.
/// Available with feature `solver-highs`.
#[cfg(feature = "solver-highs")]
pub use good_lp::solvers::highs::highs;

/// The [SCIP](https://scipopt.org/) MILP solver.
/// Available with features `solver-scip` or `solver-scip-bundled`.
#[cfg(feature = "solver-scip")]
pub use good_lp::solvers::scip::scip;

/// The [CBC](https://www.coin-or.org/Cbc/) COIN-OR solver.
/// Available with feature `solver-cbc`.
#[cfg(feature = "solver-cbc")]
pub use good_lp::solvers::coin_cbc::coin_cbc;

/// Returns an [`lp-solvers`](https://docs.rs/lp-solvers) runtime bridge to an
/// external CPLEX binary.
///
/// `binary_path` must be the absolute path to the `cplex` executable, e.g.
/// `"/usr/local/cplex/bin/x86-64_linux/cplex"`.
///
/// Available with feature `solver-lp-solvers`.
#[cfg(feature = "solver-lp-solvers")]
pub fn cplex(binary_path: &str) -> good_lp::LpSolver<lp_solvers::solvers::Cplex> {
    good_lp::LpSolver(lp_solvers::solvers::Cplex::with_command(
        binary_path.to_string(),
    ))
}

/// Returns an [`lp-solvers`](https://docs.rs/lp-solvers) runtime bridge to
/// any external solver binary that accepts `.lp` files (e.g. Gurobi, GLPK, CBC).
///
/// `solver` must be an `lp_solvers::solvers::*` value.
///
/// Available with feature `solver-lp-solvers`.
#[cfg(feature = "solver-lp-solvers")]
pub fn lp_solver<S: lp_solvers::solvers::SolverProgram>(solver: S) -> good_lp::LpSolver<S> {
    good_lp::LpSolver(solver)
}

/// The [CPLEX](https://www.ibm.com/products/ilog-cplex-optimization-studio)
/// solver via static Rust bindings (`cplex-rs` crate).
///
/// Requires a valid local CPLEX installation. Set the `CPLEX_PATH` environment
/// variable if CPLEX is not in its default installation directory.
///
/// Available with feature `solver-cplex-rs`.
#[cfg(feature = "solver-cplex-rs")]
pub use good_lp::solvers::cplex_rs::cplex_rs;
