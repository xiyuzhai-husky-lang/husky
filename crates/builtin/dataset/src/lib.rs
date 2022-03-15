mod iter;
mod labeled;
mod loader;
pub mod synthetic;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("synthetic", synthetic::SCOPE_DATA)],
    signature: StaticScopeSignature::Module,
};

use std::{borrow::Cow, sync::Arc};

pub use iter::DataIter;
pub use labeled::LabeledData;
pub use loader::DataLoader;

use scope::{BuiltinScopeData, ScopeKind, StaticScopeSignature};
use synthetic::SyntheticDataset;
use vm::{AnyValue, AnyValueDyn};

pub trait DatasetDyn: AnyValueDyn + std::fmt::Debug + Send + Sync + 'static {
    fn dev_loader(&self) -> DataLoader;
    fn val_loader(&self) -> DataLoader;
    fn test_loader(&self) -> DataLoader;
    fn profile_iter(&self) -> DataIter;
}

#[derive(Debug, Clone)]
pub struct Dataset(Arc<dyn DatasetDyn>);

impl Dataset {
    pub fn new<T: SyntheticDataset>(t: T) -> Self {
        Self(Arc::new(t))
    }

    pub fn dev_loader(&self) -> DataLoader {
        self.0.dev_loader()
    }

    pub fn val_loader(&self) -> DataLoader {
        self.0.val_loader()
    }

    pub fn test_loader(&self) -> DataLoader {
        self.0.test_loader()
    }

    pub fn profile_iter(&self) -> DataIter {
        self.0.profile_iter()
    }
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal_any(other.0.upcast_any())
    }
}

impl AnyValue for Dataset {
    fn static_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "Box<dyn Dataset>".into()
    }

    fn snapshot(&self) -> std::sync::Arc<dyn vm::AnyValueDyn> {
        todo!()
    }
}
