# MCNF Interactive Demo - Getting Started

## Overview

This is an interactive web-based demo of the `orx-network-flow` crate, showcasing the flexible solver abstraction.

## Prerequisites

- Rust 1.70+
- Node.js 18+
- npm

## Setup

### 1. Install Node dependencies

```bash
cd examples/mcnf-demo
npm install
```

### 2. Build Rust backend

```bash
cargo build --example mcnf-demo
```

## Development

### Run in dev mode

```bash
npm run tauri dev
```

This will:
- Start Vite dev server on http://localhost:5173
- Build and run the Tauri app
- Hot-reload frontend changes

### Build for production

```bash
npm run tauri build
```

## Features

### Define Problem
- Add geographic spaces (with lat/lon)
- Add commodities (origin, ready-time, destination, due-time, quantity)
- Add transports (vehicle type, routes, times, capacity)
- Add lost revenue costs per commodity

### Configure Network
- **Network Type**: Choose AOA-Wait or AON-Wait
- **Grouping Strategy**: Choose DD (Demand-Demand) or RO (Reception-Order)
- **Solver Backend**: Choose microlp (pure Rust) or CPLEX (lp-solvers)

### View Results
- Network statistics (variables, constraints, commodities, spaces, transports)
- Optimization status
- Solution details

## Architecture

### Backend (Rust)
- `src/lib.rs` - App state and module declarations
- `src/main.rs` - Tauri entry point
- `src/problem_builder.rs` - Problem construction from form input
- `src/solver_handler.rs` - Network solving logic
- `src/commands/` - Tauri IPC handlers
- `src/serialization.rs` - TypeScript ↔ Rust data bridge

### Frontend (Svelte)
- `src/App.svelte` - Main app component
- `src/components/ProblemForm.svelte` - Problem input form
- `src/components/NetworkSelector.svelte` - Network configuration
- `src/components/StatsPanel.svelte` - Statistics display
- `src/main.ts` - Svelte app entry

## Troubleshooting

### "Problem not loaded"
- Make sure to fill the form and click "Build Problem" first

### "Solver feature not enabled"
- Check your Cargo.toml features
- By default, `solver-microlp` is enabled
- Add `solver-lp-solvers` for CPLEX backend

### Build fails with missing dependencies
- Run `npm install` to ensure all Node modules are installed
- Run `cargo build` to ensure Rust dependencies are downloaded

## Future Enhancements

- [ ] Export network as DOT/SVG visualization
- [ ] Display actual solution flows
- [ ] Save/load problems as JSON
- [ ] Solve time profiling
- [ ] Support all solver backends (HiGHS, SCIP, CBC)
