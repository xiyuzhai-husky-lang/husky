#![feature(const_trait_impl)]
#![feature(const_convert)]
pub mod cv;
pub mod synthetic;

use entity_kind::TyKind;
use husky_datasets_protocol::*;
use husky_dev_utils::*;
use husky_entity_route::{EntityRouteKind, EntityRoutePtr};
use husky_liason_semantics::*;
use husky_trace_protocol::VisualData;
use husky_visual_syntax::StaticVisualTy;
use husky_word::RootIdentifier;
use serde::Serialize;
use static_defn::*;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::{borrow::Cow, sync::Arc};
use vm::__StaticInfo;
use vm::*;

extern "C" {
    pub static __DATASET_VTABLE: __RegisterVTable;
}

pub static DATASETS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "datasets",
    items: &[&synthetic::SYNTHETIC_MODULE_DEFN, &cv::CV_MOD_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::__static_dev_src!(),
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
        visual_ty: StaticVisualTy::Dataset,
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};
