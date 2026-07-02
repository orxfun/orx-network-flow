# MCNF Interactive Demo - Development Plan

## 1. Overview

**Goal**: Create an interactive web-based demo showcasing the flexibility of `orx-network-flow` solver abstraction.

**Key Features**:
- Define geographic spaces, commodities, transports, and costs via UI
- Choose network type: AOA-Wait or AON-Wait
- Choose grouping strategy: Demand-Demand (DD) or Reception-Order (RO)
- Choose solver backend: CPLEX (lp-solvers) or microlp
- Visualize network as interactive dot diagram
- Display solver statistics
- Display optimization solution

---

## 2. Architecture Decision

### Recommended: **Tauri + Svelte/Vue**

**Why Tauri?**
- Full Rust backend with direct access to `orx-network-flow` crate
- Modern web frontend (Svelte/Vue for reactive UI)
- Lightweight, native performance (~3-5MB bundle)
- Invoke Rust functions from frontend via IPC bridge
- Access to filesystem for dot diagram export

**Why Svelte?**
- Reactive, component-based frontend
- Excellent form handling and state management
- Small bundle size (~15-20KB gzipped)
- Simple learning curve for Rust developers

**Alternative Considered**: Yew (pure Rust WASM)
- **Rejected**: Heavier bundle, more complex build, less UI ecosystem

---

## 3. Project Structure

```
examples/
├── mcnf-demo/
│   ├── Cargo.toml                 # Tauri app + Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── lib.rs                # Demo state + handlers
│   │   ├── problem_builder.rs     # Interface to orx-network-flow
│   │   ├── solver_handler.rs      # Solver invocation logic
│   │   ├── serialization.rs       # JSON (de)serialization bridges
│   │   └── commands/              # Tauri command handlers
│   │       ├── problem.rs         # problem_sample() → Problem
│   │       ├── solver.rs          # solve_network() → Solution
│   │       └── stats.rs           # get_stats() → McnfStats
│   │
│   ├── src-tauri/                 # Tauri core config
│   │   └── tauri.conf.json        # Window size, features, etc.
│   │
│   └── src/                       # Svelte frontend
│       ├── App.svelte             # Main app component
│       ├── components/
│       │   ├── ProblemForm.svelte      # Geographic spaces, commodities, transports, costs
│       │   ├── NetworkSelector.svelte  # AOA/AON + DD/RO choice
│       │   ├── SolverSelector.svelte   # CPLEX vs microlp
│       │   ├── NetworkDiagram.svelte   # Display dot visualization
│       │   ├── StatsPanel.svelte       # Network statistics
│       │   └── SolutionPanel.svelte    # Solver results
│       ├── stores/
│       │   ├── problem.ts         # Problem state store
│       │   ├── network.ts         # Network choice + graph store
│       │   ├── solver.ts          # Solver state store
│       │   └── ui.ts              # UI state (loading, errors)
│       ├── types/
│       │   └── index.ts           # TypeScript interfaces (mirror Rust types)
│       ├── utils/
│       │   ├── api.ts             # Tauri command wrappers
│       │   ├── formatters.ts      # Display helpers
│       │   └── validators.ts      # Input validation
│       ├── styles/
│       │   ├── globals.css        # App-wide styles
│       │   └── components.css     # Component styles
│       └── main.ts                # Svelte app entry
│
├── README.md                      # Demo usage guide
└── DEVELOPMENT.md                 # Dev setup instructions
```

---

## 4. Phase-by-Phase Implementation Plan

### **Phase 1: Project Setup** (Est. 2-4 hours)

#### 1.1 Create Tauri project scaffold
```bash
cd examples/mcnf-demo
cargo init --name mcnf-demo
```

#### 1.2 Add Tauri dependencies to Cargo.toml
```toml
[dependencies]
tauri = { version = "2.0", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
orx-network-flow = { path = "../.." }  # Reference parent crate
tokio = { version = "1", features = ["full"] }
```

#### 1.3 Initialize Svelte frontend
```bash
npm create vite@latest . -- --template svelte
npm install
```

#### 1.4 Configure Tauri (`src-tauri/tauri.conf.json`)
- Window: 1200x800px
- DevTools in dev mode
- Build frontend before packaging

**Deliverables**:
- ✅ Tauri app boots without errors
- ✅ Frontend serves locally (localhost:5173)
- ✅ Hot reload working

---

### **Phase 2: Data Models & Serialization** (Est. 3-5 hours)

#### 2.1 Create Rust data structures for form inputs

**File**: `src/serialization.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FormGeographicSpace {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormCommodity {
    pub id: usize,
    pub origin: String,
    pub ready_time: i64,
    pub destination: String,
    pub due_time: i64,
    pub quantity: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormTransport {
    pub id: usize,
    pub vehicle_type: String,
    pub origin: String,
    pub departure_time: i64,
    pub destination: String,
    pub arrival_time: i64,
    pub capacity: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormLostRevenueCost {
    pub commodity_id: usize,
    pub cost_per_unit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ProblemInput {
    pub spaces: Vec<FormGeographicSpace>,
    pub commodities: Vec<FormCommodity>,
    pub transports: Vec<FormTransport>,
    pub lost_revenue_costs: Vec<FormLostRevenueCost>,
}
```

#### 2.2 Create TypeScript mirrors on frontend

**File**: `src/types/index.ts`

```typescript
export interface GeographicSpace {
  name: string;
  latitude: number;
  longitude: number;
}

export interface Commodity {
  id: number;
  origin: string;
  readyTime: number;
  destination: string;
  dueTime: number;
  quantity: number;
}

// ... (transports, costs, etc.)

export interface ProblemInput {
  spaces: GeographicSpace[];
  commodities: Commodity[];
  transports: Transport[];
  lostRevenueCosts: LostRevenuesCost[];
}

export interface NetworkChoice {
  networkType: "aoa" | "aon";
  groupingStrategy: "dd" | "ro";
  solverBackend: "cplex" | "microlp";
}

export interface McnfStats {
  numVariables: number;
  numConstraints: number;
  numCommodities: number;
  numSpaces: number;
  numTransports: number;
}

export interface Solution {
  status: "optimal" | "infeasible" | "unbounded";
  objectiveValue: number;
  flowArcs: FlowArc[];
}
```

**Deliverables**:
- ✅ Rust types serialize/deserialize via serde_json
- ✅ TypeScript interfaces align with Rust
- ✅ Form validation schemas ready

---

### **Phase 3: Backend Handlers** (Est. 5-7 hours)

#### 3.1 Problem Builder Wrapper

**File**: `src/problem_builder.rs`

```rust
use orx_network_flow::{Problem, ProblemBuilder};
use crate::serialization::{ProblemInput, MyVariant};

pub fn build_problem_from_input(input: ProblemInput) -> Result<Problem<MyVariant>, String> {
    let mut builder = ProblemBuilder::new();
    
    // Add geographic spaces
    builder = builder.with_geographic_spaces(
        input.spaces.iter().map(|s| (s.name, s.latitude, s.longitude))
    );
    
    // Add commodities
    for commodity in input.commodities {
        builder.push_commodity(
            commodity.id,
            commodity.origin,
            commodity.ready_time,
            commodity.destination,
            commodity.due_time,
            commodity.quantity,
        );
    }
    
    // Add transports
    for transport in input.transports {
        builder.push_transport(
            transport.id,
            12, // vehicle_type_id (fixed for demo)
            transport.vehicle_type,
            transport.origin,
            transport.departure_time,
            transport.destination,
            transport.arrival_time,
            transport.capacity,
        );
    }
    
    // Add lost revenue costs
    let mut lost_revenue = builder.lost_revenue_cost();
    for cost in input.lost_revenue_costs {
        lost_revenue.commodity_specific(&cost.commodity_id, cost.cost_per_unit);
    }
    
    Ok(builder.finish())
}
```

#### 3.2 Solver Dispatch Handler

**File**: `src/solver_handler.rs`

```rust
use orx_network_flow::{McnfSolver, Problem, Variant};
use good_lp::Solver;

pub enum NetworkConfig {
    AoaWaitDd,
    AoaWaitRo,
    AonWaitDd,
    AonWaitRo,
}

pub enum SolverChoice {
    #[cfg(feature = "solver-lp-solvers")]
    Cplex,
    #[cfg(feature = "solver-microlp")]
    Microlp,
}

pub fn solve_network<V: Variant>(
    problem: &Problem<V>,
    network: NetworkConfig,
    solver: SolverChoice,
) -> Result<Solution, String> {
    match (network, solver) {
        (NetworkConfig::AonWaitDd, SolverChoice::Cplex) => {
            // Build AON-Wait network
            // Instantiate CPLEX solver
            // Call McnfSolver::aon_wait_dd()
            // Extract solution
        },
        (NetworkConfig::AonWaitDd, SolverChoice::Microlp) => {
            // Build AON-Wait network
            // Instantiate microlp solver
            // Call McnfSolver::aon_wait_dd()
            // Extract solution
        },
        // ... (other combinations)
    }
}
```

#### 3.3 Tauri Command Handlers

**File**: `src/commands/problem.rs`

```rust
#[tauri::command]
pub async fn build_problem(input: ProblemInput) -> Result<(), String> {
    let problem = build_problem_from_input(input)?;
    // Store in AppState or return serialized
    Ok(())
}

#[tauri::command]
pub async fn get_problem_stats(problem_id: String) -> Result<ProblemStats, String> {
    // Retrieve problem and compute stats
    Ok(stats)
}
```

**File**: `src/commands/solver.rs`

```rust
#[tauri::command]
pub async fn solve_problem(
    problem_id: String,
    network_config: NetworkChoice,
) -> Result<SolutionResponse, String> {
    // 1. Retrieve problem by ID
    // 2. Construct network (AON/AOA)
    // 3. Instantiate solver (CPLEX/microlp)
    // 4. Solve
    // 5. Extract stats + solution
    Ok(response)
}

#[tauri::command]
pub async fn get_network_dot(
    problem_id: String,
    network_config: NetworkChoice,
) -> Result<String, String> {
    // Retrieve problem, build network, get .dot string
    Ok(dot_string)
}
```

**Deliverables**:
- ✅ Problem can be built from form input
- ✅ Network can be constructed with 4 configs
- ✅ Solvers dispatch correctly
- ✅ Stats extracted from solvers
- ✅ Dot diagrams generated

---

### **Phase 4: Frontend UI Components** (Est. 6-8 hours)

#### 4.1 State Management (Svelte Stores)

**File**: `src/stores/problem.ts`

```typescript
import { writable } from 'svelte/store';

export const problemStore = writable<ProblemInput>({
  spaces: [],
  commodities: [],
  transports: [],
  lostRevenueCosts: [],
});

export const problemIdStore = writable<string>("");
```

**File**: `src/stores/network.ts`

```typescript
export const networkChoiceStore = writable<NetworkChoice>({
  networkType: "aon",
  groupingStrategy: "dd",
  solverBackend: "cplex",
});

export const networkDotStore = writable<string>("");
export const statsStore = writable<McnfStats | null>(null);
```

#### 4.2 ProblemForm Component

**File**: `src/components/ProblemForm.svelte`

```svelte
<script>
  import { problemStore } from '../stores/problem';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let spaces = [];
  let commodities = [];
  let transports = [];
  let costs = [];
  
  async function handleSubmit() {
    const problem = {
      spaces,
      commodities,
      transports,
      lostRevenueCosts: costs
    };
    
    try {
      await invoke('build_problem', { input: problem });
      problemStore.set(problem);
    } catch (err) {
      console.error('Failed to build problem:', err);
    }
  }
</script>

<div class="form-container">
  <h2>Define Network Problem</h2>
  
  <!-- Geographic Spaces Input -->
  <section>
    <h3>Geographic Spaces</h3>
    {#each spaces as space, i}
      <div class="form-row">
        <input bind:value={space.name} placeholder="Space name (e.g., AMS)" />
        <input bind:value={space.latitude} type="number" placeholder="Latitude" />
        <input bind:value={space.longitude} type="number" placeholder="Longitude" />
        <button on:click={() => spaces.splice(i, 1)}>Remove</button>
      </div>
    {/each}
    <button on:click={() => spaces = [...spaces, { name: '', latitude: 0, longitude: 0 }]}>
      + Add Space
    </button>
  </section>
  
  <!-- Similar sections for Commodities, Transports, Costs -->
  
  <button on:click={handleSubmit} class="submit-btn">Build Problem</button>
</div>

<style>
  .form-container {
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 8px;
  }
  /* ... */
</style>
```

#### 4.3 NetworkSelector Component

**File**: `src/components/NetworkSelector.svelte`

```svelte
<script>
  import { networkChoiceStore } from '../stores/network';
  
  let choice = { networkType: 'aon', groupingStrategy: 'dd', solverBackend: 'cplex' };
  
  $: networkChoiceStore.set(choice);
</script>

<div class="selector">
  <h3>Network Configuration</h3>
  
  <fieldset>
    <legend>Network Type</legend>
    <label>
      <input type="radio" bind:group={choice.networkType} value="aoa" />
      AOA-Wait (Activity-On-Arc)
    </label>
    <label>
      <input type="radio" bind:group={choice.networkType} value="aon" />
      AON-Wait (Activity-On-Node)
    </label>
  </fieldset>
  
  <fieldset>
    <legend>Grouping Strategy</legend>
    <label>
      <input type="radio" bind:group={choice.groupingStrategy} value="dd" />
      Demand-Demand (DD)
    </label>
    <label>
      <input type="radio" bind:group={choice.groupingStrategy} value="ro" />
      Reception-Order (RO)
    </label>
  </fieldset>
  
  <fieldset>
    <legend>Solver Backend</legend>
    <label>
      <input type="radio" bind:group={choice.solverBackend} value="cplex" />
      CPLEX (lp-solvers)
    </label>
    <label>
      <input type="radio" bind:group={choice.solverBackend} value="microlp" />
      microlp (Pure Rust)
    </label>
  </fieldset>
</div>
```

#### 4.4 NetworkDiagram Component

**File**: `src/components/NetworkDiagram.svelte`

```svelte
<script>
  import { networkDotStore, statsStore } from '../stores/network';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let loading = false;
  let dotString = "";
  
  async function fetchDiagram() {
    loading = true;
    try {
      dotString = await invoke('get_network_dot', { /* ... */ });
      networkDotStore.set(dotString);
      // Render SVG from dot (use Graphviz-wasm or pass to backend)
    } finally {
      loading = false;
    }
  }
</script>

<div class="diagram-panel">
  <h3>Network Visualization</h3>
  <button on:click={fetchDiagram} disabled={loading}>
    {loading ? 'Loading...' : 'Generate Diagram'}
  </button>
  
  {#if dotString}
    <div id="network-diagram" class="diagram-container">
      <!-- Render SVG here (use d3-graphviz or embed SVG) -->
    </div>
  {/if}
</div>

<style>
  .diagram-container {
    border: 1px solid #ddd;
    padding: 10px;
    border-radius: 4px;
    overflow: auto;
  }
</style>
```

#### 4.5 StatsPanel & SolutionPanel Components

**File**: `src/components/StatsPanel.svelte`

```svelte
<script>
  import { statsStore } from '../stores/network';
</script>

<div class="stats-panel">
  <h3>Network Statistics</h3>
  {#if $statsStore}
    <ul>
      <li>Variables: {$statsStore.numVariables}</li>
      <li>Constraints: {$statsStore.numConstraints}</li>
      <li>Commodities: {$statsStore.numCommodities}</li>
      <li>Spaces: {$statsStore.numSpaces}</li>
      <li>Transports: {$statsStore.numTransports}</li>
    </ul>
  {/if}
</div>

<style>
  .stats-panel {
    padding: 15px;
    background: #f5f5f5;
    border-radius: 4px;
  }
</style>
```

**Deliverables**:
- ✅ All form inputs render correctly
- ✅ State flows between components
- ✅ UI responds to user selections
- ✅ Loading states and error messages

---

### **Phase 5: Integration & Visualization** (Est. 4-6 hours)

#### 5.1 Connect backend solvers to frontend

**File**: `src/commands/solver.rs` - Complete implementation

```rust
#[tauri::command]
pub async fn solve_network(
    problem_id: String,
    network_choice: NetworkChoice,
) -> Result<SolutionResponse, String> {
    // 1. Retrieve problem from store
    let problem = get_problem(problem_id)?;
    
    // 2. Construct appropriate network
    let (network, stats) = match network_choice.network_type {
        "aon" => {
            let settings = AonWaitNwSettings { add_bypass_edges: true };
            let nw = problem.construct_aon_wait_nw(settings);
            (nw, nw.stats())
        },
        "aoa" => {
            let settings = AoaWaitNwSettings { add_bypass_edges: true };
            let nw = problem.construct_aoa_wait_nw(settings);
            (nw, nw.stats())
        },
        _ => return Err("Unknown network type".into()),
    };
    
    // 3. Instantiate solver and solve
    let solution = match (network_choice.network_type, 
                          network_choice.grouping_strategy, 
                          network_choice.solver_backend) {
        ("aon", "dd", "cplex") => {
            let solver_instance = solvers::cplex("/usr/local/cplex/bin/x86-64_linux/cplex");
            let mcnf_solver = McnfSolver::aon_wait_dd(&nw, Default::default(), solver_instance);
            let stats = mcnf_solver.stats();
            mcnf_solver.solve()?
        },
        ("aon", "dd", "microlp") => {
            let mcnf_solver = McnfSolver::aon_wait_dd(&nw, Default::default(), solvers::microlp);
            let stats = mcnf_solver.stats();
            mcnf_solver.solve()?
        },
        // ... (other combinations)
        _ => return Err("Invalid configuration".into()),
    };
    
    Ok(SolutionResponse {
        stats,
        solution,
    })
}
```

#### 5.2 Graphviz Integration

Add SVG rendering:

```bash
npm install graphviz-wasm
```

**File**: `src/components/NetworkDiagram.svelte` - Updated

```svelte
<script>
  import { Graphviz } from 'graphviz-wasm';
  
  let svgElement;
  
  async function renderDot(dotString: string) {
    try {
      const graphviz = await Graphviz.load();
      const svg = graphviz.layout(dotString, 'svg', 'dot');
      svgElement.innerHTML = svg;
    } catch (err) {
      console.error('Failed to render diagram:', err);
    }
  }
</script>

<div bind:this={svgElement} class="diagram-container"></div>
```

#### 5.3 Solution Display

**File**: `src/components/SolutionPanel.svelte`

```svelte
<script>
  export let solution;
  
  function formatObjective(value: number): string {
    return value.toLocaleString('en-US', { maximumFractionDigits: 2 });
  }
</script>

<div class="solution-panel">
  <h3>Optimization Solution</h3>
  
  {#if solution}
    <div class="solution-summary">
      <p><strong>Status:</strong> {solution.status}</p>
      <p><strong>Objective Value:</strong> {formatObjective(solution.objectiveValue)}</p>
    </div>
    
    <div class="flow-details">
      <h4>Flow Allocation</h4>
      <table>
        <thead>
          <tr>
            <th>Commodity</th>
            <th>Path</th>
            <th>Flow Units</th>
          </tr>
        </thead>
        <tbody>
          {#each solution.flowArcs as arc}
            <tr>
              <td>{arc.commodity}</td>
              <td>{arc.path.join(' → ')}</td>
              <td>{arc.flow}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <p>Solve the problem to see results.</p>
  {/if}
</div>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th, td {
    padding: 8px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
</style>
```

**Deliverables**:
- ✅ Tauri commands execute solvers
- ✅ Dot diagrams render as SVG
- ✅ Solutions display correctly
- ✅ Stats update reactively

---

### **Phase 6: Testing & Polish** (Est. 3-5 hours)

#### 6.1 Unit Tests (Rust)

**File**: `src/problem_builder.rs` - Add tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_simple_problem() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace { name: "A".into(), latitude: 0.0, longitude: 0.0 },
                FormGeographicSpace { name: "B".into(), latitude: 1.0, longitude: 1.0 },
            ],
            commodities: vec![],
            transports: vec![],
            lost_revenue_costs: vec![],
        };
        
        let problem = build_problem_from_input(input);
        assert!(problem.is_ok());
    }
}
```

#### 6.2 Integration Tests

Test each solver + network combination compiles and runs.

#### 6.3 UI Polish

- Add loading spinners
- Improve error messages
- Add form validation feedback
- Responsive layout for smaller screens
- Dark mode support (optional)

**Deliverables**:
- ✅ All solvers tested
- ✅ UI error handling
- ✅ Responsive design
- ✅ Documentation

---

## 5. Dependencies & Features

### Cargo.toml additions

```toml
[dev-dependencies]
tauri = { version = "2", features = ["shell-open", "protocol-asset"] }

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt"] }
```

### package.json additions

```json
{
  "devDependencies": {
    "@tauri-apps/api": "^2.0",
    "@tauri-apps/cli": "^2.0",
    "vite": "^5.0",
    "svelte": "^4.0",
    "graphviz-wasm": "^1.0"
  }
}
```

---

## 6. Implementation Timeline

| Phase | Duration | Cumulative |
|-------|----------|-----------|
| 1. Setup | 2-4h | 2-4h |
| 2. Data Models | 3-5h | 5-9h |
| 3. Backend Handlers | 5-7h | 10-16h |
| 4. UI Components | 6-8h | 16-24h |
| 5. Integration | 4-6h | 20-30h |
| 6. Testing & Polish | 3-5h | 23-35h |
| **Total** | — | **23-35 hours** |

---

## 7. Success Criteria

- [ ] User can input problem via form UI
- [ ] Problem builds successfully
- [ ] User can select network type (AOA/AON) and grouping (DD/RO)
- [ ] User can select solver (CPLEX/microlp)
- [ ] Network visualizes correctly as SVG
- [ ] Network statistics display (variables, constraints, etc.)
- [ ] Solver runs without errors
- [ ] Solution displays with objective value and flow allocation
- [ ] All 4 solver feature combinations work (1 × 2 × 2)
- [ ] Code is well-documented
- [ ] Demo runs on both Linux and macOS (if applicable)

---

## 8. Future Enhancements (Post-MVP)

1. **Persistence**: Save/load problems as JSON
2. **Export**: Generate problem reports (PDF/HTML)
3. **Comparison**: Side-by-side solver comparison (CPLEX vs microlp)
4. **Profiling**: Display solve time per configuration
5. **Interactive Network**: Click nodes/edges to highlight flows
6. **Multi-language**: Translate UI to German/Dutch
7. **API Mode**: REST API instead of desktop app
8. **Tutorials**: Guided walkthroughs for first-time users

---

## 9. Risks & Mitigation

| Risk | Mitigation |
|------|-----------|
| Tauri build complexity | Provide setup scripts; document env vars |
| Graphviz rendering slow | Cache rendered diagrams; show loading states |
| Large networks crash UI | Pagination/filtering; disable features for 1000+ nodes |
| CPLEX not installed | Fallback to microlp; warn user in UI |
| Serialization mismatches | Extensive type tests; keep TypeScript in sync |

---

## 10. Next Steps

1. **Initialize project** (`cargo new examples/mcnf-demo`)
2. **Create file structure** as specified in Section 3
3. **Start Phase 1**: Tauri + Svelte scaffolding
4. **Weekly sync**: Review progress on each phase
5. **Deploy**: Package as standalone app once complete

