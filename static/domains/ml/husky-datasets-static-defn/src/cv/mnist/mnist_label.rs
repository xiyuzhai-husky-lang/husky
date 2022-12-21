use husky_dev_utils::*;
use husky_entity_kind::TyKind;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_trace_protocol::VisualData;

pub static MNIST_LABEL_BASE_ROUTE: &'static str = "cv::datasets::mnist::MnistLabel";

macro_rules! enum_variant_defns {
    ($($variant: ident),*) => {{
        [$(EntityStaticDefn {
            name: stringify!($variant),
            items: &[],
            variant: EntityStaticDefnVariant::EnumVariant,
            dev_src: static_dev_src!(),
        }),*]
    }};
}

pub static MNIST_LABEL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "MnistLabel",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: MNIST_LABEL_BASE_ROUTE,
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &enum_variant_defns!(Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine),
        kind: TyKind::Enum,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|_| Ok(VisualData::void())),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
