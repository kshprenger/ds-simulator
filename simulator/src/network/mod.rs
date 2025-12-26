mod bandwidth;
mod latency;

use std::cell::RefMut;
use std::collections::BTreeMap;
use std::rc::Rc;

pub(crate) use bandwidth::BandwidthQueue;
pub(crate) use bandwidth::BandwidthQueueOptions;
pub use bandwidth::BandwidthType;
pub(crate) use latency::LatencyQueue;
use log::debug;

use crate::Configuration;
use crate::Destination;
use crate::Message;
use crate::MessagePtr;
use crate::ProcessHandle;
use crate::ProcessId;
use crate::access;
use crate::actor::SimulationActor;
use crate::communication::ProcessStep;
use crate::communication::RoutedMessage;
use crate::process::SharedProcessHandle;
use crate::random::Randomizer;
use crate::random::Seed;
use crate::time::Jiffies;
use crate::time::Now;

pub(crate) struct Network {
    bandwidth_queue: BandwidthQueue,
    procs: BTreeMap<ProcessId, SharedProcessHandle>,
}

impl Network {
    fn SubmitSingleMessage(
        &mut self,
        message: Rc<dyn Message>,
        source: ProcessId,
        destination: Destination,
        base_arrival_time: Jiffies,
    ) {
        let targets = match destination {
            Destination::Broadcast => self.procs.keys().copied().collect::<Vec<ProcessId>>(),
            Destination::To(to) => vec![to],
        };

        debug!("Submitting message from {source}, targets of the message: {targets:?}",);

        targets.into_iter().for_each(|target| {
            let routed_message = RoutedMessage {
                arrival_time: base_arrival_time,
                step: ProcessStep {
                    source,
                    dest: target,
                    message: message.clone(),
                },
            };
            self.bandwidth_queue.Push(routed_message);
        });
    }

    fn HandleOf(&mut self, process_id: ProcessId) -> RefMut<'_, Box<dyn ProcessHandle>> {
        self.procs
            .get_mut(&process_id)
            .expect("Invalid proccess id")
            .borrow_mut()
    }

    fn ExecuteProcessStep(&mut self, step: ProcessStep) {
        let source = step.source;
        let dest = step.dest;
        let message = step.message;

        debug!(
            "Executing step for process {} | Message Source: {}",
            dest, source
        );

        access::SetProcess(dest);

        self.HandleOf(dest)
            .OnMessage(source, MessagePtr::New(message));
    }
}

impl Network {
    pub(crate) fn New(
        seed: Seed,
        max_network_latency: Jiffies,
        bandwidth_type: BandwidthType,
        procs: BTreeMap<ProcessId, SharedProcessHandle>,
    ) -> Self {
        Self {
            bandwidth_queue: BandwidthQueue::New(
                bandwidth_type,
                procs.len(),
                LatencyQueue::New(Randomizer::New(seed), max_network_latency),
            ),
            procs,
        }
    }

    pub(crate) fn SubmitMessages(
        &mut self,
        messages: &mut Vec<(ProcessId, Destination, Rc<dyn Message>)>,
    ) {
        messages
            .drain(..)
            .into_iter()
            .for_each(|(from, destination, message)| {
                self.SubmitSingleMessage(message, from, destination, Now() + Jiffies(1));
            });
    }
}

impl SimulationActor for Network {
    fn Start(&mut self) {
        for id in self.procs.keys().copied().collect::<Vec<ProcessId>>() {
            debug!("Executing initial step for {id}");
            let config = Configuration {
                assigned_id: id,
                proc_num: self.procs.keys().len(),
            };

            access::SetProcess(id);

            self.HandleOf(id).Bootstrap(config);
        }
    }

    fn Step(&mut self) {
        let next_event = self.bandwidth_queue.Pop();

        match next_event {
            BandwidthQueueOptions::None => {}
            BandwidthQueueOptions::MessageArrivedByLatency => {}
            BandwidthQueueOptions::Some(message) => {
                self.ExecuteProcessStep(message.step);
            }
        }
    }

    fn PeekClosest(&self) -> Option<Jiffies> {
        self.bandwidth_queue.PeekClosest()
    }
}
