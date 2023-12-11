pub use ad_hoc_task_dependency_macros::*;

#[macro_export]
macro_rules! init_crate {
    () => {};
}

#[macro_export]
macro_rules! linkages {
    ($($linkage: expr),*,) => {
        const LINKAGES: &'static [::ad_hoc_task_dependency::Linkage]
        = &[
            $($linkage.into()),*
        ];
    }
}

pub struct Linkage {}
