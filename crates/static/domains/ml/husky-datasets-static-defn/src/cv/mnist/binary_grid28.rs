use super::*;
use husky_datasets_interface::mnist::BinaryGrid28;
use husky_liason_semantics::{MemberLiason, ParameterLiason};
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_trace_protocol::*;
use std::any::TypeId;
use vm::*;

pub static BINARY_GRID_28_BASE_ROUTE: &'static str =
    "domains::ml::datasets::cv::mnist::BinaryGrid28";

pub static BINARY_GRID_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: BINARY_GRID_28_BASE_ROUTE,
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
                            BinaryGrid28,
                            __BINARY_GRID_28_VTABLE,
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
            visual_ty: StaticVisualTy::Shape2d,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: Some(&BINARY_GRID28_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static BINARY_GRID28_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::None,
        output_ty: "domains::ml::datasets::cv::mnist::BinaryGrid28",
        output_liason: OutputLiason::Transfer,
        linkage: transfer_linkage!(|_, _values|unsafe  {
            (__Register::new_box(BinaryGrid28::default(), &__BINARY_GRID_28_VTABLE))
        },
        some BinaryGrid28::__call__)
        .into(),
    },
    dev_src: static_dev_src!(),
};
