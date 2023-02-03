use super::*;
use husky_datasets_interface::mnist::BinaryGrid28;
use husky_liason_semantics::{MemberModifier, ParameterModifier};
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer, StaticVisualizerFp};
use husky_vm::*;

pub static BINARY_GRID_28_BASE_ROUTE: &'static str = "cv::datasets::mnist::BinaryGrid28";

pub static BINARY_GRID_28_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "BinaryGrid28",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: BINARY_GRID_28_BASE_ROUTE,
        spatial_parameters: &[],
        trait_impls: &[StaticTraitImplDefn {
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
                        return_ty: "r32",
                        output_liason: OutputModifier::MemberAccess {
                            member_liason: MemberModifier::Mutable,
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
        variadic_template: StaticVariadicParameterDecl::None,
        return_ty: "cv::datasets::mnist::BinaryGrid28",
        output_liason: OutputModifier::Transfer,
        linkage: transfer_linkage!(
            |_, _values| __Register::new_box(BinaryGrid28::default(), &__BINARY_GRID_28_VTABLE),
            some base BinaryGrid28::__call__ as fn() -> BinaryGrid28
        )
        .into(),
    },
    dev_src: static_dev_src!(),
};
