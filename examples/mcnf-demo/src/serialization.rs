use serde::{Deserialize, Serialize};

/// Geographic space with coordinates
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormGeographicSpace {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Commodity request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormCommodity {
    pub id: usize,
    pub origin: String,
    pub ready_time: i64,
    pub destination: String,
    pub due_time: i64,
    pub quantity: u64,
}

/// Transport option
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormTransport {
    pub id: usize,
    pub vehicle_type: String,
    pub origin: String,
    pub departure_time: i64,
    pub destination: String,
    pub arrival_time: i64,
    pub capacity: u64,
}

/// Lost revenue cost for commodity
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FormLostRevenueItem {
    pub commodity_id: usize,
    pub cost_per_unit: i64,
}

/// Complete problem input from frontend
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProblemInput {
    pub spaces: Vec<FormGeographicSpace>,
    pub commodities: Vec<FormCommodity>,
    pub transports: Vec<FormTransport>,
    pub lost_revenue_costs: Vec<FormLostRevenueItem>,
}

/// Network configuration choice
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkChoice {
    pub network_type: String,      // "aoa" or "aon"
    pub grouping_strategy: String, // "dd" or "ro"
    pub solver_backend: String,    // "cplex" or "microlp"
}

/// Path routing information for a commodity
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommodityPath {
    pub path_index: usize,
    pub flow: u64,
    pub num_transports: usize,
    /// Path represented as transport indices: "0-1-2"
    pub transport_path: String,
    /// Path represented as space sequence: "AMS-BRU-LEJ"
    pub space_path: String,
    /// Path represented as vertex indices (space-time nodes): "0-1-2-3"
    pub vertex_path: String,
}

/// Commodity routing solution
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommoditySolution {
    pub commodity_id: usize,
    pub paths: Vec<CommodityPath>,
    pub total_flow: u64,
}

/// Transport utilization information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportUtilization {
    pub transport_id: usize,
    pub total_load: u64,
    pub num_commodities: usize,
}

/// Complete solution data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolutionData {
    pub commodity_solutions: Vec<CommoditySolution>,
    pub transport_utilizations: Vec<TransportUtilization>,
    pub total_flow_routed: u64,
}

/// Network response with statistics and solution data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct McnfResponse {
    pub num_variables: usize,
    pub num_constraints: usize,
    pub num_commodities: usize,
    pub num_spaces: usize,
    pub num_transports: usize,
    #[serde(default)]
    pub objective_value: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub solution_data: Option<SolutionData>,
}

/// Solution response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolutionResponse {
    pub status: String, // "optimal", "infeasible", "unbounded"
    pub objective_value: f64,
}
