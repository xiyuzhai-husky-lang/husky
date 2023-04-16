use husky_static_visualizer::StaticVisualizer;

use crate::*;

pub static TYPE_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "TypeType",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "Type",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::HigherKind,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
pub static TRAIT_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "TraitType",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "Trait",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::HigherKind,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
pub static MODULE_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "ModuleType",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "Module",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::HigherKind,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Void,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
