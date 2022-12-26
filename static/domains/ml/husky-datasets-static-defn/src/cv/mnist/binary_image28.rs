use crate::*;
pub use husky_datasets_interface::mnist::BinaryImage28;
use husky_static_visualizer::{StaticVisualizer, StaticVisualizerFp};

pub static BINARY_IMAGE_28_ROUTE: &'static str = "cv::datasets::mnist::BinaryImage28";

pub static BINARY_IMAGE_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: BINARY_IMAGE_28_ROUTE,
        spatial_parameters: &[],
        trait_impls: &[
            clone_trait_impl!(BINARY_IMAGE_28_ROUTE),
            StaticTraitImplDefn {
                dev_src: static_dev_src!(),
                trai: "std::ops::Index<i32>",
                member_impls: &[
                    associated_type_impl!("Output", "r32"),
                    EntityStaticDefn {
                        dev_src: husky_dev_utils::static_dev_src!(),
                        name: "index",
                        items: &[],
                        variant: EntityStaticDefnVariant::Method {
                            this_modifier: ParameterModifier::MemberAccess,
                            parameters: &[StaticParameter {
                                modifier: ParameterModifier::None,
                                ty: "i32",
                                name: "todo!()",
                            }],
                            output_ty: "r32",
                            output_liason: OutputModifier::MemberAccess {
                                member_liason: MemberModifier::Mutable,
                            },
                            spatial_parameters: &[],
                            method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                            opt_linkage: Some(index_linkage!(
                                mutable,
                                Intrinsic,
                                Direct,
                                BinaryImage28,
                                __BINARY_IMAGE_28_VTABLE,
                                u32,
                                __B32_VTABLE
                            )),
                        },
                    },
                ],
            },
        ],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Image2d,
            fp: StaticVisualizerFp(|value| {
                let value: &BinaryImage28 = value.downcast_temp_ref(&__BINARY_IMAGE_28_VTABLE);
                Ok(VisualData::BinaryImage28 {
                    padded_rows: value.padded_rows.clone(),
                })
            }),
        },
        opt_type_call: Some(&BINARY_IMAGE28_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static BINARY_IMAGE28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicParameterDecl::None,
        output_ty: BINARY_IMAGE_28_ROUTE,
        output_liason: OutputModifier::Transfer,
        linkage: transfer_linkage!(
            |_, _values| __Register::new_box(BinaryImage28::default(), &__BINARY_IMAGE_28_VTABLE),
            some base BinaryImage28::__call__ as fn() -> BinaryImage28
        )
        .into(),
    },
    dev_src: static_dev_src!(),
};
