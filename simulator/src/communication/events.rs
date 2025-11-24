use std::collections::HashSet;

use priority_queue::PriorityQueue;

use crate::{process::ProcessId, time::Jiffies};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum Event {
    Timeout,
    Message(Message),
}

pub type EventBatch = HashSet<(Destination, Event)>;

#[macro_export]
macro_rules! events {
    [] => {
        std::collections::HashSet::new()
    };
    [$($dest:expr => $event:expr),+ $(,)?] => {
        {
            let mut set = std::collections::HashSet::new();
            $(
                set.insert(($dest, $event));
            )*
            set
        }
    };
}

impl Event {
    pub(crate) fn size(&self) -> usize {
        match self {
            Event::Timeout => 0,
            Event::Message(msg) => msg.payload.len(),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum Destination {
    Broadcast,
    SendSelf,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Message {
    pub payload: bytes::Bytes,
}

/// ((ProcessId, Event), Jiffies) <=> At specified timestamp event will be delivered with source of ProcessId
pub type TimePriorityEventQueue = PriorityQueue<(ProcessId, Event), Jiffies>;
