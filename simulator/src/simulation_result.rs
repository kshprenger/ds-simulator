use crate::{history::ExecutionHistory, metrics::Metrics};

pub enum SimulationResult {
    Ok(Metrics),
    Deadlock(ExecutionHistory),
}
