use crate::{
    metrics::{self, Metrics},
    random,
};

struct Simulation {}

impl Simulation {
    fn new(seed: random::Seed) -> Self {
        Self {}
    }
    fn start(&mut self) {}
    fn stop(&mut self) -> metrics::Metrics {
        Metrics {}
    }
}
