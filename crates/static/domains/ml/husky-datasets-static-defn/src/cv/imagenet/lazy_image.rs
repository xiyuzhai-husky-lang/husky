use super::*;

pub struct LazyImage {}

pub static LAZY_IMAGE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "LazyImage",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "domains::ml::datasets::cv::imagenet::LazyImage",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Image2d,
            fp: StaticVisualizerFp(|value| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
