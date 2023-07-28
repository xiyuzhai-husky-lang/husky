mod collect_refs;
mod cyclic_slice_;
mod firstx;
mod lastx;
mod pop_with_largest_opt_f32;

pub use collect_refs::*;
pub use cyclic_slice_::*;
pub use firstx::*;
pub use lastx::*;
pub use pop_with_largest_opt_f32::VEC_POP_WITH_LARGEST_OPT_F32;

use super::*;
use husky_print_utils::msg_once;
use husky_static_visualizer::StaticVisualTy;

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "Vec",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
            dev_src: static_dev_src!(),
        }],
        trait_impls: &[
            clone_trait_impl!("Vec"),
            StaticTraitImplDefn {
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
                            opt_linkage: Some(__LinkageGroup::Member(&__MemberLinkageGroup {
                                copy_resolved_linkage: resolved_linkage!(virtual_vec_index_copy),
                                eval_ref_resolved_linkage: resolved_linkage!(
                                    virtual_vec_index_eval_ref
                                ),
                                temp_ref_resolved_linkage: resolved_linkage!(
                                    virtual_vec_index_temp_ref
                                ),
                                temp_mut_resolved_linkage: resolved_linkage!(
                                    virtual_vec_index_temp_mut
                                ),
                                move_resolved_linkage: resolved_linkage!(virtual_vec_index_move),
                            })),
                        },
                    },
                ],
                dev_src: static_dev_src!(),
            },
        ],
        ty_members: &[
            &VEC_LEN,
            &VEC_PUSH,
            &VEC_POPX,
            &VEC_FIRST,
            &VEC_LAST,
            &VEC_COLLECT_REFS,
            &VEC_CYCLIC_SLICE,
            &VEC_POP_WITH_LARGEST_OPT_F32,
        ],
        variants: &[],
        kind: TyKind::Vec,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Group,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicParameterDecl::RepeatSingle(StaticParameter {
            modifier: ParameterModifier::Owned,
            ty: "E",
            name: "items",
        }),
        return_ty: "Vec<E>",
        output_liason: OutputModifier::Transfer,
        linkage: transfer_linkage!(virtual_list_type_call, none),
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_list_type_call<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let mut data = vec![];
    for value in values {
        data.push(value.bind_move())
    }
    __Register::new_box(
        DeprecatedVirtualVec::new(data),
        &__DEPRECATED_VIRTUAL_VEC_VTABLE,
    )
}

unsafe fn virtual_vec_push<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let element = values[1].bind_move();
    let virtual_vec: &mut DeprecatedVirtualVec =
        values[0].downcast_temp_mut(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    virtual_vec.push(element);
    ().to_register()
}

unsafe fn virtual_vec_pop<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    let virtual_vec: &mut DeprecatedVirtualVec =
        values[0].downcast_temp_mut(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    virtual_vec.pop().unwrap()
}

fn virtual_vec_index_move<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    todo!()
}

unsafe fn virtual_vec_index_copy<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__DEPRECATED_VIRTUAL_VEC_VTABLE as *const _
    );
    let this_value: &DeprecatedVirtualVec =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].bind_copy()
}

unsafe fn virtual_vec_index_eval_ref<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    assert_eq!(
        values[0].vtable as *const _,
        &__DEPRECATED_VIRTUAL_VEC_VTABLE as *const _
    );
    let this_value: &'eval DeprecatedVirtualVec =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].eval_bind_eval_ref()
}

unsafe fn virtual_vec_index_temp_ref<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    msg_once!("the current impl of virtual vec is deprecated");
    let this_value: &DeprecatedVirtualVec =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    let i: usize = values[1].downcast_i32() as usize;
    this_value[i].bind_temp_ref()
}

unsafe fn virtual_vec_index_temp_mut<'eval>(
    values: &mut [__Register<'eval>],
    _opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let i: usize = values[1].downcast_i32() as usize;
    let this_value: &mut DeprecatedVirtualVec =
        values[0].downcast_temp_mut(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].bind_temp_mut()
}

pub static VEC_LEN: EntityStaticDefn = EntityStaticDefn {
    name: "ilen",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "i32",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_len, none)),
        output_liason: OutputModifier::Transfer,
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_len<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_vec: &DeprecatedVirtualVec =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    let len: i32 = virtual_vec.len().try_into().unwrap();
    len.to_register()
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
    name: "push",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::TempRefMut,
        parameters: &[StaticParameter {
            modifier: ParameterModifier::Owned,
            ty: "E",
            name: "element",
        }],
        return_ty: "void",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_push, none)),
        output_liason: OutputModifier::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_POPX: EntityStaticDefn = EntityStaticDefn {
    name: "popx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::TempRefMut,
        parameters: &[],
        return_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_pop, none)),
        output_liason: OutputModifier::Transfer,
    },
    dev_src: static_dev_src!(),
};
