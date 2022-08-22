use husky_trace_protocol::VisualData;

use super::*;

pub static FP_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "FatFp",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "FatFp",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::FatFp,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::FatFp,
            fp: StaticVisualizerFp(|value| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
