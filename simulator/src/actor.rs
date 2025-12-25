use crate::time::Jiffies;

pub(crate) trait SimulationActor {
    fn Start(&mut self);
    fn Step(&mut self);
    fn PeekClosest(&self) -> Option<Jiffies>;
}
