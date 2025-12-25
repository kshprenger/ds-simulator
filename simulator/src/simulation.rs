use core::time;
use std::process::exit;

use log::{error, info};

use crate::{
    actor::SimulationActor,
    network::{BandwidthType, Network},
    process::{ProcessId, SharedProcessHandle},
    progress::Bar,
    random::{self},
    time::{FastForwardClock, Jiffies, Now},
};

pub struct Simulation {
    actors: Vec<Box<dyn SimulationActor>>,
    max_time: Jiffies,
    progress_bar: Bar,
}

const K_PROGRESS_TIMES: usize = 10;

impl Simulation {
    pub(crate) fn New(
        seed: random::Seed,
        max_time: Jiffies,
        max_network_latency: Jiffies,
        bandwidth_type: BandwidthType,
        procs: Vec<(ProcessId, SharedProcessHandle)>,
    ) -> Self {
        let _ = env_logger::try_init();

        let mut actors = Vec::new();
        let network_actor = Box::new(Network::New(
            seed,
            max_network_latency,
            bandwidth_type,
            procs.into_iter().collect(),
        )) as Box<dyn SimulationActor>;

        actors.push(network_actor);

        Self {
            actors,
            max_time,
            progress_bar: Bar::New(max_time, max_time.0 / K_PROGRESS_TIMES),
        }
    }

    pub fn Run(&mut self) {
        self.Start();

        while self.KeepRunning() {
            match self.PeekClosest() {
                None => {
                    error!("DEADLOCK! (ﾉಥ益ಥ）ﾉ ┻━┻ Try with RUST_LOG=debug");
                    exit(1)
                }
                Some((time, actor)) => {
                    FastForwardClock(time);
                    actor.Step();
                    self.progress_bar.MakeProgress(Now());
                }
            }
        }

        info!("Looks good! ヽ(‘ー`)ノ");
    }
}

impl Simulation {
    fn KeepRunning(&mut self) -> bool {
        Now() < self.max_time
    }

    fn Start(&mut self) {
        self.actors.iter_mut().for_each(|actor| actor.Start());
    }

    fn PeekClosest(&mut self) -> Option<(Jiffies, &mut Box<dyn SimulationActor>)> {
        self.actors
            .iter_mut()
            .filter_map(|actor| Some((actor.PeekClosest()?, actor)))
            .min_by_key(|tuple| tuple.0)
    }
}
