use bytes::Bytes;
use simulator::{
    Destination, Event, EventSet, Jiffies, ProcessHandle, ProcessId, SimulationBuilder, event_set,
};

struct ExampleProcess {}

impl ExampleProcess {
    fn new() -> Self {
        Self {}
    }
}

impl ProcessHandle for ExampleProcess {
    fn init(&mut self) -> EventSet {
        event_set![Destination::SendSelf => Event::Timeout]
    }

    fn on_event(&mut self, event: (ProcessId, Event)) -> EventSet {
        match event.1 {
            Event::Timeout => {
                event_set![Destination::Broadcast => Event::Message(Bytes::new())]
            }
            Event::Message(_) => {
                event_set![Destination::SendSelf => Event::Timeout]
            }
        }
    }
}

fn main() {
    SimulationBuilder::new_with_process_factory(|| Box::new(ExampleProcess::new()))
        .with_network_bandwidth(simulator::BandwidthType::Unbounded)
        .with_max_network_latency(Jiffies(2))
        .with_max_steps(Jiffies(100_000))
        .with_process_count(200)
        .with_seed(5)
        .build()
        .run();

    println!("Done")
}
