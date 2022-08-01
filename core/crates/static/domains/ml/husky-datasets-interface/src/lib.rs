mod __rust_code_gen__;
pub mod cv;
mod iter;
mod labeled;
mod loader;
pub mod synthetic;

pub use __rust_code_gen__::*;
pub use cv::*;
use husky_trace_protocol::VisualData;
pub use iter::*;
pub use labeled::*;
pub use loader::*;
pub use synthetic::*;

use husky_entity_route::{EntityRouteKind, EntityRoutePtr};
use husky_word::RootIdentifier;
use serde::Serialize;
use std::{
    borrow::Cow,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::Arc,
};
use vm::*;

pub trait DatasetDyn<'eval>:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'eval
{
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
        todo!()
        // self.0.__eq__(other.0.__upcast_any())
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
impl<'a> __StaticInfo for Dataset<'a> {
    type __StaticSelf = Dataset<'static>;

    fn __static_typename() -> Cow<'static, str> {
        "Dataset".into()
    }
}
