mod iter;
mod loader;
pub mod synthetic;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("synthetic", synthetic::SCOPE_DATA)],
    call_signature: None,
    compiled: None,
};
pub use iter::SampleIter;
pub use loader::SampleLoader;

use scope::{BuiltinScopeData, ScopeKind};
use std::any::Any;
use virtual_stack::VirtualStack;

pub trait Dataset {
    fn dev_loader(&self) -> SampleLoader;
    fn val_iter(&self) -> SampleIter;
    fn test_iter(&self) -> SampleIter;
}

pub enum Input<'a> {
    Owned(Box<dyn Any>),
    Borrowed(&'a dyn Any),
}
