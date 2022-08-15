use crate::*;
pub use husky_datasets_interface::mnist::BinaryImage28;
use husky_static_visualizer::{StaticVisualizer, StaticVisualizerFp};

pub static BINARY_IMAGE_28_BASE_ROUTE: &'static str =
    "domains::ml::datasets::cv::mnist::BinaryImage28";

pub static BINARY_IMAGE_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryImage28",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: BINARY_IMAGE_28_BASE_ROUTE,
        spatial_parameters: &[],
        trait_impls: &[StaticTraitImplDefn {
            dev_src: static_dev_src!(),
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "b32"),
                EntityStaticDefn {
                    dev_src: husky_dev_utils::static_dev_src!(),
                    name: "index",
                    items: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_liason: ParameterLiason::MemberAccess,
                        parameters: &[StaticParameter {
                            liason: ParameterLiason::Pure,
                            ty: "i32",
                            name: "todo!()",
                        }],
                        output_ty: "b32",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
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
        }],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Image2d,
            fp: StaticVisualizerFp(|value| {
                let value: &BinaryImage28 =
                    unsafe { value.downcast_temp_ref(&__BINARY_IMAGE_28_VTABLE) };
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
        variadic_template: StaticVariadicTemplate::None,
        output_ty: BINARY_IMAGE_28_BASE_ROUTE,
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(|_, _values| unsafe {
            (__Register::new_box(BinaryImage28::default(), &__BINARY_IMAGE_28_VTABLE))
        }, some BinaryImage28::__call__)
        .into(),
    },
    dev_src: static_dev_src!(),
};
