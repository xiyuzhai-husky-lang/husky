mod iter;
mod labeled;
mod loader;
mod synthetic;

use husky_trace_protocol::VisualData;
pub use iter::*;
pub use labeled::*;
pub use loader::*;
pub use synthetic::*;

use husky_entity_route::{EntityRouteKind, EntityRoutePtr};
use serde::Serialize;
use std::{borrow::Cow, sync::Arc};
use vm::*;
use word::RootIdentifier;

pub trait DatasetDyn<'eval>: AnyValueDyn<'eval> + std::fmt::Debug + Send + Sync + 'eval {
    fn dev_loader(&self) -> DataLoader<'eval>;
    fn val_loader(&self) -> DataLoader<'eval>;
    fn test_loader(&self) -> DataLoader<'eval>;
    fn profile_iter(&self) -> DataIter<'eval>;
}

#[derive(Debug, Clone)]
pub struct Dataset<'eval>(Arc<dyn DatasetDyn<'eval>>);

impl<'eval> Dataset<'eval> {
    pub fn new<T: DatasetDyn<'eval>>(t: T) -> Self {
        Self(Arc::new(t))
    }

    pub fn dev_loader(&self) -> DataLoader<'eval> {
        self.0.dev_loader()
    }

    pub fn val_loader(&self) -> DataLoader<'eval> {
        self.0.val_loader()
    }

    pub fn test_loader(&self) -> DataLoader<'eval> {
        self.0.test_loader()
    }

    pub fn profile_iter(&self) -> DataIter<'eval> {
        self.0.profile_iter()
    }
}

impl<'eval> PartialEq for Dataset<'eval> {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal_any(other.0.upcast_any())
    }
}

impl<'eval> Serialize for Dataset<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
impl<'a> HasStaticTypeInfo for Dataset<'a> {
    type StaticSelf = Dataset<'static>;

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval, 'a: 'eval> AnyValue<'eval> for Dataset<'a> {
    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::DatasetType.into()
    }

    fn print_short(&self) -> String {
        todo!()
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<husky_trace_protocol::VisualData>> {
        todo!()
    }
}
