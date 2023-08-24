use small_cell_stack::SmallCellStack;
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    thread::LocalKey,
};

pub trait Task: RefUnwindSafe + UnwindSafe {
    type DevAscension: Ascension;
    type Ascension: Ascension;
}

// E -> B
pub trait Ascension {
    type Base: AscensionBase;
    type RuntimeStorage: RefUnwindSafe + UnwindSafe;
}

pub trait AscensionBase {
    const DIMENSION: u8;
    type ClosedPoint: Copy + 'static;
    const CLOSED_POINT_STACK: &'static LocalKey<SmallCellStack<[Self::ClosedPoint; 8]>>;
}
