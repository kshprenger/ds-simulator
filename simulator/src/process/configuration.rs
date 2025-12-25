use std::{cell::RefCell, rc::Rc};

use crate::ProcessHandle;

pub(crate) type SharedProcessHandle = Rc<RefCell<Box<dyn ProcessHandle>>>;

pub type ProcessId = usize;

pub struct Configuration {
    pub assigned_id: ProcessId,
    pub proc_num: usize,
}
