use crate::{EventSet, communication::Event};

pub type ProcessId = usize;

pub trait ProcessHandle {
    /// Should schedule some initial events
    fn init(&mut self) -> EventSet;

    /// Deliver event with source process
    fn on_event(&mut self, event: (ProcessId, Event)) -> EventSet;
}
