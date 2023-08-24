use husky_hir_deps::HirLinkageDeps;
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
    type LibraryStorage: Default + RefUnwindSafe + UnwindSafe;
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
    type LinkageKey: Copy + Eq + std::hash::Hash;
    type Linkage;
    fn get_linkage(&self, key: Self::LinkageKey) -> Option<(HirLinkageDeps, Self::Linkage)>;
}

pub type DevLinkageTable<Task: IsTask> = <Task::DevAscension as IsAscension>::LinkageTable;
pub type DevLibraryStorage<Task: IsTask> = <Task::DevAscension as IsAscension>::LibraryStorage;
pub type DevRuntimeTaskSpecificConfig<Task: IsTask> =
    <Task::DevAscension as IsAscension>::RuntimeTaskSpecificConfig;
pub type DevRuntimeStorage<Task: IsTask> = <Task::DevAscension as IsAscension>::RuntimeStorage;
