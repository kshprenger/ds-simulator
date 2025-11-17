use crate::communication::{Destination, Message};
use std::collections::HashSet;

pub trait Node {
    fn on_message(&mut self, m: Message) -> HashSet<(Destination, Message)>;
    fn tick(&mut self) -> HashSet<(Destination, Message)>;
}
