use crate::{communication::Event, process::ProcessId};

/// (ProcessId, Event, ProcessId) <=> (Source, Event, Destination)
pub(crate) type ProcessStep = (ProcessId, Event, ProcessId);
pub(crate) type ExecutionHistory = Vec<ProcessStep>;
