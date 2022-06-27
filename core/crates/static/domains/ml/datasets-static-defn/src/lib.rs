#![feature(const_trait_impl)]
#![feature(const_convert)]
pub mod cv;
mod iter;
mod labeled;
mod loader;

pub use labeled::*;
pub mod synthetic;

use liason::*;
use visual_syntax::{primitive_visualizer, StaticVisualTy};
use word::RootIdentifier;

pub static DATASETS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "datasets",
    items: &[&synthetic::SYNTHETIC_MODULE_DEFN, &cv::CV_MOD_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub static DATASET_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Dataset",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Dataset",
        spatial_parameters: &[
            StaticSpatialParameter {
                name: "Input",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            },
            StaticSpatialParameter {
                name: "Output",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            },
        ],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Other,
        visualizer: &primitive_visualizer(StaticVisualTy::Dataset),
        opt_type_call: None,
    },
    dev_src: dev_utils::static_dev_src!(),
};

use std::{borrow::Cow, sync::Arc};

use entity_kind::TyKind;
pub use iter::DataIter;
pub use labeled::LabeledData;
pub use loader::{DataLoader, LoadSample};

use entity_route::{EntityRouteKind, EntityRoutePtr};
use serde::Serialize;
use static_defn::*;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use vm::{AnyValue, AnyValueDyn, HasStaticTypeInfo};

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

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::DatasetType.into()
    }
}
