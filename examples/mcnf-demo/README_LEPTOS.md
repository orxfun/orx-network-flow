# MCNF Demo - Leptos Frontend

This project uses **Leptos**, a full-stack Rust framework with WebAssembly compilation for the frontend.

## Architecture

- **Backend**: Rust library (`src/lib.rs`) with domain logic for problem building and solving
- **Frontend**: Leptos SPA compiled to WebAssembly using Trunk
- **State**: Shared `AppState` for managing current problem data
- **IPC**: Ready for Tauri integration (commands already defined in `src/commands/`)

## Development

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (build tool for Leptos)
cargo install trunk
```

### Running Locally

```bash
# Start dev server (Trunk serves on http://localhost:3000)
npm run dev
# or
trunk serve

# In another terminal, optionally run tests
cargo test
```

### Building for Production

```bash
# Build optimized WASM bundle
npm run build
# or
trunk build --release

# Output: ../dist/ (ready for Tauri or static hosting)
```

## Project Structure

```
src/
├── lib.rs                 # Library root with state and module exports
├── main.rs               # Leptos entry point (compiles to WASM)
├── app.rs                # Main App component and sub-components
├── styles.css            # Global styles
├── problem_builder.rs    # Build Problem from form input
├── serialization.rs      # Form data types (TypeScript ↔ Rust)
├── solver_handler.rs     # Network solving logic
└── commands/
    └── problem.rs        # Tauri command handlers (future use)

index.html                # HTML template for Trunk
Trunk.toml               # Trunk configuration
Cargo.toml               # Rust dependencies and features
package.json             # NPM scripts (for convenience)
```

## Features

### Leptos Components

1. **App** - Main layout with 3-section grid
2. **ProblemForm** - Input form for spaces, commodities, transports, costs
3. **NetworkSelector** - Configuration UI (network type, grouping, solver)
4. **StatsPanel** - Display network statistics

### Solver Backends

Features can be enabled for different solvers:

```bash
# Default (MicroLP)
cargo build --target wasm32-unknown-unknown

# With HiGHS
cargo build --target wasm32-unknown-unknown --features "std,solver-highs"

# With CBC
cargo build --target wasm32-unknown-unknown --features "std,solver-cbc"
```

## Notes

- WASM compilation can take time; first build may take 2-3 minutes
- Leptos signals are reactive; UI updates automatically
- State management uses `create_signal()` for reactivity
- The backend library can be used independently for CLI or other frontends

## Next Steps

1. Connect form inputs to actual problem building (currently placeholder)
2. Implement actual solver invocation based on network/solver selection
3. Add problem statistics extraction from solver instances
4. Wire Tauri commands for desktop app integration
5. Add more visualization (network diagrams, solution paths)
