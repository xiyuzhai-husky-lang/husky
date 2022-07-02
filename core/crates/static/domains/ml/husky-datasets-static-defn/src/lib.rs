#![feature(const_trait_impl)]
#![feature(const_convert)]
pub mod cv;
pub mod synthetic;

use dev_utils::*;
use entity_kind::TyKind;
use husky_datasets_protocol::*;
use husky_entity_route::{EntityRouteKind, EntityRoutePtr};
use husky_liason_semantics::*;
use husky_visual_syntax::{primitive_visualizer, StaticVisualTy};
use serde::Serialize;
use static_defn::*;
use static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use std::{borrow::Cow, sync::Arc};
use vm::*;
use vm::{AnyValue, AnyValueDyn, HasStaticTypeInfo};
use word::RootIdentifier;

pub static DATASETS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "datasets",
    items: &[&synthetic::SYNTHETIC_MODULE_DEFN, &cv::CV_MOD_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::__static_dev_src!(),
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
    dev_src: dev_utils::__static_dev_src!(),
};
