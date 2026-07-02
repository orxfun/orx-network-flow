# Solver Abstraction Plan

## Goal

Allow users of `orx-network-flow` to choose their LP/MILP solver via Cargo features,
rather than being locked in to CPLEX via the `lp-solvers` runtime bridge. The
abstraction is already largely in place: every MCNF solver is generic over
`S: good_lp::Solver`. The work is therefore mostly in `Cargo.toml` and in
providing convenient constructors or re-exports for each solver.

---

## Current State

```toml
# Cargo.toml (current)
[dependencies]
good_lp   = { version = "1.15.2", features = ["lp-solvers"], default-features = false }
lp-solvers = { version = "1.2", features = ["cplex"] }
```

```rust
// src/mcnf/factory.rs (current)
type StatsSolver = good_lp::solvers::lp_solvers::LpSolver<lp_solvers::solvers::Cplex>;
```

```rust
// examples/shared/shared_problem.rs (current)
pub fn cplex_solver() -> LpSolver<Cplex> {
    good_lp::LpSolver(Cplex::with_command(
        "/usr/local/cplex/bin/x86-64_linux/cplex".to_string(),
    ))
}
```

The solver is passed as a generic type parameter through every solver struct
(e.g. `AonWaitRoMcnfSolver<'a, V, S>`), so **no internal solver API changes are
needed**. All changes are in:

1. `Cargo.toml` — feature declarations and conditional dependencies
2. `src/mcnf/factory.rs` — the hardcoded `StatsSolver` alias
3. `examples/shared/shared_problem.rs` — example helper
4. Optionally, a new `src/solvers.rs` re-export module to give users
   ready-made solver constructors without having to reach into `good_lp` internals.

---

## Solver Options

`good_lp` already provides a unified `Solver` trait backed by many engines.
All of the solvers below are available through `good_lp` features — no need
to depend on lower-level crates (`highs-sys`, `cplex-rs-sys`) directly.

| Feature name (ours)   | `good_lp` feature | External requirement | License | MILP | Notes |
|-----------------------|--------------------|----------------------|---------|------|-------|
| `solver-microlp`      | `microlp`          | None (pure Rust)     | MIT     | No   | Recommended default; works in `no_std`, compiles to WASM |
| `solver-highs`        | `highs`            | C++ stdlib + cmake at build time (statically linked) | MIT | Yes | Fast open-source MILP; parallel |
| `solver-scip`         | `scip` + `scip_bundled` | None (bundled binary) | Apache-2.0 | Yes | Best open-source MILP; easiest install via `scip_bundled` |
| `solver-cbc`          | `coin_cbc`         | `coinor-cbc` system lib | EPL | Yes | Former default of `good_lp` |
| `solver-lp-solvers`   | `lp-solvers`       | External binary (cplex, gurobi, glpk, cbc…) at runtime | MIT | Yes | Current approach; runtime overhead |
| `solver-cplex-rs`     | `cplex-rs`         | Local CPLEX installation + clang/llvm | Commercial | Yes | Static link to CPLEX via `github.com/cplex-rs/cplex-rs`; fastest commercial option |

### Notes on the repos mentioned

- **`highs` crate** (`crates.io/crates/highs`): this is the safe Rust binding
  that `good_lp` already uses internally when you enable `features = ["highs"]`.
  You do not need to add it as a direct dependency — enabling `solver-highs`
  in `orx-network-flow` is sufficient.

- **`highs-sys` crate** (`crates.io/crates/highs-sys`): this is the raw
  unsafe FFI layer beneath `highs`. Useful only if you need low-level HiGHS
  control. Not needed here.

- **`cplex-rs/cplex-rs` repo** (`github.com/cplex-rs/cplex-rs`): this **is**
  the same crate that `good_lp` uses. The crate is published to crates.io as
  `cplex-rs = "0.1.9"`, owned by Matteo Biggio (`mbiggio`), and hosted under
  the `cplex-rs` GitHub organisation. Enabling `good_lp`'s `cplex-rs` feature
  pulls it in directly — no custom fork or extra integration needed.

- **`lp-solvers` + CPLEX** (current): writes the problem to a `.lp` file and
  spawns a CPLEX process. Adds hundreds of milliseconds per solve. Useful
  for users who have CPLEX but prefer not to compile against its C library.

---

## Proposed `Cargo.toml` Changes

```toml
[dependencies]
good_lp = { version = "1.15.2", default-features = false }

# Each solver pulled in only when its feature is active
lp-solvers = { version = "1.2", features = ["cplex"], optional = true }

[features]
default = ["std", "solver-microlp"]

std = []
serde-export = ["dep:serde", "dep:serde_json"]

# ── Solver features ───────────────────────────────────────────────────────────
# Pure-Rust, zero external deps. Good default and for tests.
solver-microlp     = ["good_lp/microlp"]

# HiGHS: free, fast, open-source MILP. Requires C++ compiler + cmake at build time.
solver-highs       = ["good_lp/highs"]

# SCIP: best open-source MILP solver.
# Enable solver-scip-bundled to ship a precompiled SCIP binary (easier).
solver-scip        = ["good_lp/scip"]
solver-scip-bundled = ["solver-scip", "good_lp/scip_bundled"]

# CBC: COIN-OR branch-and-cut. Requires coinor-cbc system library.
solver-cbc         = ["good_lp/coin_cbc"]

# lp-solvers: runtime bridge to external solvers (CPLEX, Gurobi, GLPK, CBC).
# Solver binary must be installed separately by the end user.
solver-lp-solvers  = ["good_lp/lp-solvers", "dep:lp-solvers"]

# CPLEX via static Rust bindings (requires local CPLEX installation + clang).
# Mutually exclusive with solver-lp-solvers (both ultimately link cplex-rs-sys).
solver-cplex-rs    = ["good_lp/cplex-rs"]
```

> **Note:** `lpsolve` and `cplex-rs` are mutually exclusive in `good_lp`
> (they conflict at the linker level). Do not enable both simultaneously.
> This should be documented in the crate README but does not require a
> compile-time `cfg` check in `orx-network-flow` since `good_lp` itself
> will emit an error.

---

## Code Changes

### 1. `src/mcnf/factory.rs` — Remove the hardcoded `StatsSolver`

The `StatsSolver` alias is used only by the `*_stats` methods to compute model
statistics without a real solver. Replace it with `microlp` (always available
in tests and CI since it is the default feature) guarded by a cfg:

```rust
// Before
type StatsSolver = good_lp::solvers::lp_solvers::LpSolver<lp_solvers::solvers::Cplex>;

// After — use whichever solver is compiled in, checked at build time
#[cfg(feature = "solver-microlp")]
type StatsSolver = good_lp::solvers::microlp::MicroLpSolver;

#[cfg(all(feature = "solver-highs", not(feature = "solver-microlp")))]
type StatsSolver = good_lp::solvers::highs::HighsSolver;

// … additional fallbacks as needed, or simply require solver-microlp for stats
```

The simplest approach: **require `solver-microlp` for the `*_stats` methods**
and gate them with `#[cfg(feature = "solver-microlp")]`. This is acceptable
because these methods are used only in tests and tooling, not in production
solving paths.

### 2. `src/utils/math_model.rs` — LP file export

The `lp_solvers_model_to_lp_file` function uses an `unsafe` cast that
assumes the model is an `lp_solvers::problem::Problem`. This is only valid
when `solver-lp-solvers` is active. Gate it:

```rust
#[cfg(feature = "solver-lp-solvers")]
pub unsafe fn lp_solvers_model_to_lp_file<S: Solver, P>(...) { ... }
```

### 3. New module: `src/solvers.rs` (optional but recommended)

Provide ready-made constructor functions so that users need not know `good_lp`
internals:

```rust
// src/solvers.rs

/// Returns the built-in pure-Rust solver.
/// Available with feature `solver-microlp`.
#[cfg(feature = "solver-microlp")]
pub fn microlp() -> good_lp::solvers::microlp::MicroLpSolver {
    good_lp::solvers::microlp::MicroLpSolver
}

/// Returns the HiGHS solver.
/// Available with feature `solver-highs`.
#[cfg(feature = "solver-highs")]
pub fn highs() -> good_lp::solvers::highs::HighsSolver {
    good_lp::solvers::highs::HighsSolver
}

/// Returns the SCIP solver.
/// Available with feature `solver-scip`.
#[cfg(feature = "solver-scip")]
pub fn scip() -> good_lp::solvers::scip::SCIPSolver {
    good_lp::solvers::scip::SCIPSolver::default()
}

/// Returns an lp-solvers CPLEX bridge.
/// Available with feature `solver-lp-solvers`.
#[cfg(feature = "solver-lp-solvers")]
pub fn cplex(binary_path: &str) -> good_lp::LpSolver<lp_solvers::solvers::Cplex> {
    good_lp::LpSolver(lp_solvers::solvers::Cplex::with_command(binary_path.to_string()))
}

/// Returns the CPLEX solver via static Rust bindings.
/// Available with feature `solver-cplex-rs`.
#[cfg(feature = "solver-cplex-rs")]
pub fn cplex_rs() -> good_lp::solvers::cplex_rs::CplexSolver {
    good_lp::solvers::cplex_rs::CplexSolver::default()
}
```

Expose via `pub use solvers;` in `lib.rs`.

### 4. Update examples

`examples/shared/shared_problem.rs` should be updated to use either the
new `orx_network_flow::solvers::*` helpers or demonstrate multiple solver
choices, one per example or selected via env var.

---

## Migration for Existing Users

Users currently calling `cplex_solver()` from the shared example need to:

1. Add `features = ["solver-lp-solvers"]` to their `orx-network-flow`
   dependency (or keep the default `solver-microlp` for testing).
2. Replace `cplex_solver()` with `orx_network_flow::solvers::cplex("/path/to/cplex")`.

---

## Recommended Test Setup

In `Cargo.toml` dev-dependencies or CI matrix:

```toml
[dev-dependencies]
# no extra: solver-microlp is in the default feature set
```

CI should run:
- `cargo test` (uses `microlp` by default — pure Rust, no external installs)
- `cargo test --no-default-features --features solver-highs` (if HiGHS is available)
- `cargo test --no-default-features --features solver-scip-bundled` (downloads SCIP binary)

---

## Implementation Sequence

1. **Update `Cargo.toml`** — add feature declarations, switch `good_lp` to
   `default-features = false`, make `lp-solvers` optional.
2. **Fix `StatsSolver` in `factory.rs`** — replace the CPLEX-specific alias
   with a microlp alias gated on `solver-microlp`.
3. **Gate `math_model.rs` helpers** on `solver-lp-solvers`.
4. **Add `src/solvers.rs`** with convenience constructors.
5. **Update `src/lib.rs`** to re-export `solvers`.
6. **Update examples** to use `orx_network_flow::solvers::*`.
7. **Verify compilation** for each feature flag combination with
   `cargo check --no-default-features --features solver-<X>`.
8. **Update README** to document the feature flags.
