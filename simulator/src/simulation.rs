use std::{collections::HashMap, rc::Rc};

use crate::{
    communication::{Event, EventDeliveryQueue},
    metrics::{self, Metrics},
    process::{ProcessHandle, ProcessId},
    random::{self, Randomizer},
    time::Jiffies,
};

pub struct Simulation {
    randomizer: Randomizer,
    procs: HashMap<ProcessId, (Box<dyn ProcessHandle>, EventDeliveryQueue)>,
    metrics: Metrics,
    global_time: Jiffies,
    max_steps: Jiffies,
}

impl Simulation {
    pub(crate) fn new(seed: random::Seed, max_steps: Jiffies) -> Self {
        Self {
            randomizer: Randomizer::new(seed),
            procs: HashMap::new(),
            metrics: Metrics {},
            global_time: 0,
            max_steps: max_steps,
        }
    }

    pub(crate) fn submit_event(&mut self, event: Event) {
        match event {
            Event::Timeout(after) => {}
            Event::Message(message) => {
                todo!()
            }
        }
    }

    pub(crate) fn add_processes(&mut self, procs: Vec<Box<dyn ProcessHandle>>) {
        procs.into_iter().enumerate().for_each(|(id, proc)| {
            self.procs.insert(id, (proc, EventDeliveryQueue::new()));
        });
    }

    pub(crate) fn run(&mut self) -> metrics::Metrics {
        self.initial_step();

        while self.keep_running() {
            if !self.step() {
                panic!("Deadlock")
            }
            self.tick();
        }

        self.metrics.clone()
    }
}

impl Simulation {
    fn keep_running(&self) -> bool {
        self.global_time < self.max_steps
    }

    fn tick(&mut self) {
        self.global_time += 1;
    }

    fn initial_step(&mut self) {
        self.procs.values_mut().for_each(|(process_handle, _)| {
            process_handle.init();
        });
    }

    fn step(&mut self) -> bool {
        let next_events = self.choose_next_events();
        if next_events.is_empty() {
            return false;
        }
        self.deliver_events(next_events);
        return true;
    }

    fn deliver_events(&mut self, events: Vec<(ProcessId, Event)>) {
        events.into_iter().for_each(|(target, event)| {
            self.procs
                .get_mut(&target)
                .expect("Process not found")
                .0
                .on_event(event);
        })
    }

    fn choose_next_events(&mut self) -> Vec<(ProcessId, Event)> {
        self.procs
            .iter_mut()
            .filter(|(_, (_, candidate_queue))| !candidate_queue.is_empty())
            .map(|(candidate, (_, candidate_queue))| {
                (candidate, candidate_queue.pop().expect("Queue is empty"))
            })
            .filter(|(_, (arrival_time, _))| *arrival_time == self.global_time)
            .map(|(candidate, (_, event))| (candidate.clone(), event))
            .collect()
    }
}
