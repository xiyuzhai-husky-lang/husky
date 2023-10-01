#![feature(const_trait_impl)]
#![feature(const_convert)]
pub mod cv;
pub mod synthetic;

use husky_datasets_interface::{__rust_code_gen__::*, *};
use husky_dev_utils::*;
use husky_entity_kind::TyKind;
use husky_liason_semantics::*;
use husky_static_defn::*;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_trace_protocol_old::VisualData;
use husky_vm::*;
use serde::Serialize;
use std::borrow::Cow;

pub static DATASETS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "datasets",
    items: &[&synthetic::SYNTHETIC_MODULE_DEFN, &cv::CV_MOD_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static DATASET_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Dataset",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "Dataset",
        spatial_parameters: &[
            StaticSpatialParameter {
                name: "Input",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
                dev_src: static_dev_src!(),
            },
            StaticSpatialParameter {
                name: "Output",
                variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
                dev_src: static_dev_src!(),
            },
        ],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::BoxAny,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Dataset,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
