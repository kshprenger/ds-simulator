use crate::random;

pub type SimulationMain = dyn FnOnce(random::Seed) -> ();
