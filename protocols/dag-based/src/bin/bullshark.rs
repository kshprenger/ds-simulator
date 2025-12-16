use dag_based::bullshark::Bullshark;
use simulator::{BandwidthType, Jiffies, SimulationBuilder};
fn main() {
    for procs in (4..100).step_by(10) {
        let mut sim = SimulationBuilder::NewFromFactory(|| Bullshark::New())
            .MaxLatency(Jiffies(234))
            .MaxTime(Jiffies(234567))
            .NetworkBandwidth(BandwidthType::Bounded(47))
            .ProcessInstances(procs)
            .Seed(69)
            .Build();
        let metrics = sim.Run();
        println!("{}, {}", procs, metrics.events_total)
    }
}
