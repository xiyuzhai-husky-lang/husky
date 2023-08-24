use small_cell_stack::SmallCellStack;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    thread::LocalKey,
};

pub trait IsTask: RefUnwindSafe + UnwindSafe {
    type DevAscension: IsAscension;
    type Ascension: IsAscension;
}

// E -> B
pub trait IsAscension {
    type Base: IsAscensionBase;
    type RuntimeStorage: Default + RefUnwindSafe + UnwindSafe;
    type RuntimeTaskSpecificConfig: Default + RefUnwindSafe + UnwindSafe;
}

pub trait IsAscensionBase {
    const DIMENSION: u8;
    type ClosedPoint: Copy + 'static;
    const CLOSED_POINT_STACK: &'static LocalKey<SmallCellStack<[Self::ClosedPoint; 8]>>;
}

pub type DevRuntimeTaskSpecificConfig<Task: IsTask> =
    <Task::DevAscension as IsAscension>::RuntimeTaskSpecificConfig;
pub type DevRuntimeStorage<Task: IsTask> = <Task::DevAscension as IsAscension>::RuntimeStorage;
