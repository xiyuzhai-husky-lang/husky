mod iter;
mod labeled;
mod loader;
pub mod synthetic;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("synthetic", synthetic::SCOPE_DATA)],
    signature: RawScopeSignature::Module,
};

use std::borrow::Cow;

pub use iter::DataIter;
pub use labeled::LabeledData;
pub use loader::DataLoader;

use scope::{BuiltinScopeData, RawScopeSignature, ScopeKind};
use vm::AnyValue;

pub trait Dataset: std::fmt::Debug + Send + Sync + 'static {
    fn dev_loader(&self) -> DataLoader;
    fn val_loader(&self) -> DataLoader;
    fn test_loader(&self) -> DataLoader;
    fn profile_iter(&self) -> DataIter;
}

impl AnyValue for Box<dyn Dataset> {
    fn static_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn static_type_name() -> Cow<'static, str> {
        "Box<dyn Dataset>".into()
    }
}
