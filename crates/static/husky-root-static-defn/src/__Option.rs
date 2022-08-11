use husky_trace_protocol::VisualData;

use super::*;

pub static OPTION_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Option",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Option",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Fp,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|value| todo!()),
        },
        opt_type_call: None, // shouldn't be none actually
    },
    dev_src: static_dev_src!(),
};
