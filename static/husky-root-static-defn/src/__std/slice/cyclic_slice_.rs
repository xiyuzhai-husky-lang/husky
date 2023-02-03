mod end;
mod index;
mod start;

use end::*;
use husky_static_visualizer::StaticVisualTy;
use husky_vm::*;
use index::*;
use start::*;

use super::*;
pub static STD_SLICE_CYCLIC_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "CyclicSlice",
    items: &[],
    variant: EntityStaticDefnVariant::Term {
        base_route: "std::slice::CyclicSlice",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            dev_src: static_dev_src!(),
        }],
        ty_members: &[
            &STD_SLICE_CYCLIC_SLICE_START_DEFN,
            &STD_SLICE_CYCLIC_SLICE_END_DEFN,
            &STD_SLICE_CYCLIC_SLICE_FIRST_DEFN,
            &STD_SLICE_CYCLIC_SLICE_LAST_DEFN,
        ],
        trait_impls: &[StaticTraitImplDefn {
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "E"),
                EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "index",
                    items: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_modifier: ParameterModifier::MemberAccess,
                        parameters: &[],
                        return_ty: "E",
                        output_liason: OutputModifier::MemberAccess {
                            member_liason: MemberModifier::Mutable,
                        },
                        spatial_parameters: &[],
                        method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
                            copy_resolved_linkage: resolved_linkage!(
                                virtual_cyclic_slice_index_copy
                            ),
                            eval_ref_resolved_linkage: resolved_linkage!(
                                virtual_cyclic_slice_index_eval_ref
                            ),
                            temp_ref_resolved_linkage: resolved_linkage!(
                                virtual_cyclic_slice_index_temp_ref
                            ),
                            temp_mut_resolved_linkage: resolved_linkage!(
                                virtual_cyclic_slice_index_temp_mut
                            ),
                            move_resolved_linkage: resolved_linkage!(
                                virtual_cyclic_slice_index_move
                            ),
                        })),
                    },
                },
            ],
            dev_src: static_dev_src!(),
        }],
        variants: &[],
        kind: TyKind::CyclicSlice,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Group,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "firstx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::MemberAccess,
        parameters: &[],
        return_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_first_copy),
            eval_ref_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_first_eval_ref),
            temp_ref_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_first_temp_ref),
            temp_mut_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_first_temp_mut),
            move_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_first_move),
        })),
        output_liason: OutputModifier::MemberAccess {
            member_liason: MemberModifier::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_cyclic_slice_first_copy<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_cyclic_slice_first_eval_ref<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_cyclic_slice: &'eval DeprecatedVirtualCyclicSlice<'eval> =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.first().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_cyclic_slice_first_temp_ref<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_cyclic_slice: &DeprecatedVirtualCyclicSlice<'eval> =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.first().unwrap().bind_temp_ref()
}

fn virtual_cyclic_slice_first_temp_mut<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!("deprecated")
}

fn virtual_cyclic_slice_first_move<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "lastx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::MemberAccess,
        parameters: &[],
        return_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_last_copy),
            eval_ref_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_last_eval_ref),
            temp_ref_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_last_temp_ref),
            temp_mut_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_last_temp_mut),
            move_resolved_linkage: resolved_linkage!(virtual_cyclic_slice_last_move),
        })),
        output_liason: OutputModifier::MemberAccess {
            member_liason: MemberModifier::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_cyclic_slice_last_copy<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_cyclic_slice_last_eval_ref<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_cyclic_slice: &'eval DeprecatedVirtualCyclicSlice<'eval> =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.last().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_cyclic_slice_last_temp_ref<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_cyclic_slice: &DeprecatedVirtualCyclicSlice<'eval> =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.last().unwrap().bind_temp_ref()
}

fn virtual_cyclic_slice_last_temp_mut<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!("deprecated")
}

fn virtual_cyclic_slice_last_move<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}
