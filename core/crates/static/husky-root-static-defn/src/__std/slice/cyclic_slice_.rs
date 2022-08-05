mod end;
mod index;
mod start;

use ::cyclic_slice::CyclicSlice;
use end::*;
use husky_static_visualizer::StaticVisualTy;
use index::*;
use start::*;
use std::any::TypeId;
use vm::*;

use super::*;
pub static STD_SLICE_CYCLIC_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "CyclicSlice",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "std::slice::CyclicSlice",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
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
                        this_liason: ParameterLiason::MemberAccess,
                        parameters: &[],
                        output_ty: "E",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
                        },
                        spatial_parameters: &[],
                        method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
                            copy_fp: linkage_fp!(virtual_cyclic_slice_index_copy),
                            eval_ref_fp: linkage_fp!(virtual_cyclic_slice_index_eval_ref),
                            temp_ref_fp: linkage_fp!(virtual_cyclic_slice_index_temp_ref),
                            temp_mut_fp: linkage_fp!(virtual_cyclic_slice_index_temp_mut),
                            move_fp: linkage_fp!(virtual_cyclic_slice_index_move),
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
        },
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "firstx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_fp: linkage_fp!(virtual_cyclic_slice_first_copy),
            eval_ref_fp: linkage_fp!(virtual_cyclic_slice_first_eval_ref),
            temp_ref_fp: linkage_fp!(virtual_cyclic_slice_first_temp_ref),
            temp_mut_fp: linkage_fp!(virtual_cyclic_slice_first_temp_mut),
            move_fp: linkage_fp!(virtual_cyclic_slice_first_move),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_cyclic_slice_first_copy<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_cyclic_slice_first_eval_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_cyclic_slice: &'eval VirtualCyclicSlice<'eval> =
        values[0].downcast_eval_ref(&__VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.first().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_cyclic_slice_first_temp_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_cyclic_slice: &VirtualCyclicSlice<'eval> =
        values[0].downcast_temp_ref(&__VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.first().unwrap().bind_temp_ref()
}

fn virtual_cyclic_slice_first_temp_mut<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!("deprecated")
}

fn virtual_cyclic_slice_first_move<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "lastx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_fp: linkage_fp!(virtual_cyclic_slice_last_copy),
            eval_ref_fp: linkage_fp!(virtual_cyclic_slice_last_eval_ref),
            temp_ref_fp: linkage_fp!(virtual_cyclic_slice_last_temp_ref),
            temp_mut_fp: linkage_fp!(virtual_cyclic_slice_last_temp_mut),
            move_fp: linkage_fp!(virtual_cyclic_slice_last_move),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_cyclic_slice_last_copy<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_cyclic_slice_last_eval_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_cyclic_slice: &'eval VirtualCyclicSlice<'eval> =
        values[0].downcast_eval_ref(&__VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.last().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_cyclic_slice_last_temp_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_cyclic_slice: &VirtualCyclicSlice<'eval> =
        values[0].downcast_temp_ref(&__VIRTUAL_CYCLIC_SLICE_VTABLE);
    virtual_cyclic_slice.last().unwrap().bind_temp_ref()
}

fn virtual_cyclic_slice_last_temp_mut<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!("deprecated")
}

fn virtual_cyclic_slice_last_move<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}
