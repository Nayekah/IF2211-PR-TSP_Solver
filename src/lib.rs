pub mod graph;
pub mod tsp_solver;
pub mod visualization;
pub mod config;

pub use graph::Graph;
pub use tsp_solver::TSPSolver;
pub use visualization::Visualizer;
pub use config::ConfigReader;

#[derive(Debug)]
pub enum TSPError {
    InvalidGraph(String),
    InvalidPath(String),
    ConfigError(String),
    SolverError(String),
}

impl std::fmt::Display for TSPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TSPError::InvalidGraph(msg) => write!(f, "Invalid Graph: {}", msg),
            TSPError::InvalidPath(msg) => write!(f, "Invalid Path: {}", msg),
            TSPError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            TSPError::SolverError(msg) => write!(f, "Solver Error: {}", msg),
        }
    }
}

impl std::error::Error for TSPError {}

pub type Result<T> = std::result::Result<T, TSPError>;