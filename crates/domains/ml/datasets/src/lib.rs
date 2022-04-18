pub mod cv;
mod iter;
mod labeled;
mod loader;
pub mod synthetic;

pub static DATASETS_MODULE_DEFN: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[
        ("synthetic", synthetic::SCOPE_DATA),
        ("cv", &cv::SCOPE_DATA),
    ],
    decl: StaticEntityDecl::Module,
};

pub static DATASET_TYPE_DEFN: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[],
    decl: StaticEntityDecl::Type(&StaticTypeDecl {
        base_route: "Dataset",
        generic_placeholders: &[
            StaticGenericPlaceholder {
                name: "Input",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            },
            StaticGenericPlaceholder {
                name: "Output",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            },
        ],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Other,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
};

use std::{borrow::Cow, sync::Arc};

use entity_kind::TyKind;
pub use iter::DataIter;
pub use labeled::LabeledData;
pub use loader::{DataLoader, LoadSample};

use entity_route::EntityRouteKind;
use static_decl::*;
use static_defn::StaticEntityDefn;
use visual_syntax::TRIVIAL_VISUALIZER;
use vm::{AnyValue, AnyValueDyn, HuskyBuiltinStaticTypeId, StaticTypeId};

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

impl<'eval> AnyValue<'eval> for Dataset<'eval> {
    fn static_type_id() -> StaticTypeId {
        HuskyBuiltinStaticTypeId::Dataset.into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "Arc<dyn Dataset>".into()
    }

    fn snapshot(&self) -> std::sync::Arc<dyn vm::AnyValueDyn<'eval>> {
        todo!()
    }
}
