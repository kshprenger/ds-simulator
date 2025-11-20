use std::collections::BinaryHeap;

use crate::{process::ProcessId, time::Jiffies};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    Timeout(Jiffies),
    Message(Message),
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Message {
    source: ProcessId,
    payload: bytes::Bytes,
}

/// (Jiffies, Event) <=> At speciffied timestamp event will be delivered
pub type EventDeliveryQueue = BinaryHeap<(Jiffies, Event)>;
