mod end;
mod start;

use ::cyclic_slice::CyclicSlice;
use end::*;
use start::*;
use std::any::TypeId;
use visual_syntax::{StaticVisualTy, StaticVisualizerVariant};
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
        static_trait_impls: &[],
        variants: &[],
        kind: TyKind::Struct,
        visualizer: &StaticVisualizer {
            ty: StaticVisualTy::Group,
            variant: StaticVisualizerVariant::CyclicSlice,
        },
        opt_type_call: None,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::MemberAccess {
            copy_access: routine_linkage!(generic_cyclic_slice_first_copy, 1),
            eval_ref_access: routine_linkage!(generic_cyclic_slice_first_eval_ref, 1),
            temp_ref_access: routine_linkage!(generic_cyclic_slice_first_temp_ref, 1),
            temp_mut_access: routine_linkage!(generic_cyclic_slice_first_mut, 1),
            move_access: routine_linkage!(generic_cyclic_slice_first_move, 1),
        }),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_first_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_first_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &VirtualCyclicSlice<'eval> = values[0].downcast_ref();
    match generic_cyclic_slice.first() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &VirtualCyclicSlice<'eval> = values[0].downcast_ref();
    match generic_cyclic_slice.first() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!("deprecated")
}

fn generic_cyclic_slice_first_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "last",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::MemberAccess {
            copy_access: routine_linkage!(generic_cyclic_slice_last_copy, 1),
            eval_ref_access: routine_linkage!(generic_cyclic_slice_last_eval_ref, 1),
            temp_ref_access: routine_linkage!(generic_cyclic_slice_last_temp_ref, 1),
            temp_mut_access: routine_linkage!(generic_cyclic_slice_last_mut, 1),
            move_access: routine_linkage!(generic_cyclic_slice_last_move, 1),
        }),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_last_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_last_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &VirtualCyclicSlice<'eval> = values[0].downcast_ref();
    match generic_cyclic_slice.last() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &VirtualCyclicSlice<'eval> = values[0].downcast_ref();
    match generic_cyclic_slice.last() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!("deprecated")
}

fn generic_cyclic_slice_last_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> EvalResult<TempValue<'temp, 'eval>> {
    todo!()
}
