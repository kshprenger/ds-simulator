use crate::{Message, ProcessId, SimulationAccess, process::Configuration};

pub trait ProcessHandle<M: Message> {
    /// This methods provides initial configuration to the process. That currently includes only assigned ProcessId.
    /// It is also requires process to schedule some initial events.
    fn Bootstrap(&mut self, configuration: Configuration, access: &mut SimulationAccess<M>);

    /// Deliver event with source process
    fn OnMessage(&mut self, from: ProcessId, message: M, access: &mut SimulationAccess<M>);
}
