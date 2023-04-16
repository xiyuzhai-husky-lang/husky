use super::*;

pub struct LazyImage256 {
    data: LazyImage256Inner,
}

impl LazyImage256 {
    pub fn inner(&self) -> &LazyImage256Inner {
        todo!()
    }
}

pub type LazyImage256Inner = [[[u8; 3]; 256]; 256];

pub static LAZY_IMAGE256_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "LazyImage256",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "cv::datasets::imagenet::LazyImage256",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Image2d,
            fp: StaticVisualizerFp(|_value| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
