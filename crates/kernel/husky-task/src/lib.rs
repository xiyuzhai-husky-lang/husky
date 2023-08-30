use husky_hir_deps::{HirDepsDb, HirLinkageDeps, HirLinkageKey};
use husky_regular_value::__RegularValue;
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
    type LinkageTable: IsLinkageTable;
    type RuntimeStorage: Default + RefUnwindSafe + UnwindSafe;
    type RuntimeTaskSpecificConfig: Default + RefUnwindSafe + UnwindSafe;
}

pub trait IsAscensionBase {
    const DIMENSION: u8;
    type ClosedPoint: Copy + 'static;
    const CLOSED_POINT_STACK: &'static LocalKey<SmallCellStack<[Self::ClosedPoint; 8]>>;
}

pub trait IsLinkageTable: RefUnwindSafe + UnwindSafe {
    type Linkage: IsLinkage;
    // linkage table has the responsibility to guarantee that the linkage provided is up to date.
    fn get_linkage(&self, key: HirLinkageKey, db: &dyn HirDepsDb) -> Self::Linkage;
}

pub type DevLinkageTable<Task: IsTask> = <Task::DevAscension as IsAscension>::LinkageTable;
pub type DevRuntimeTaskSpecificConfig<Task: IsTask> =
    <Task::DevAscension as IsAscension>::RuntimeTaskSpecificConfig;
pub type DevRuntimeStorage<Task: IsTask> = <Task::DevAscension as IsAscension>::RuntimeStorage;

pub trait IsLinkage: UnwindSafe + RefUnwindSafe + Copy {
    fn eval_fn();
    fn eval_gn() -> __RegularValue;
}
