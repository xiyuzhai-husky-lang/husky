use super::*;

pub static FP_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ThickFp",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "ThickFp",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::ThickFp,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::ThickFp,
            fp: StaticVisualizerFp(|value| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
