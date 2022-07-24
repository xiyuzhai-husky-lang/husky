mod cyclic_slice_;
mod firstx;
mod lastx;

use cyclic_slice::CyclicSlice;
pub use cyclic_slice_::*;
pub use firstx::*;
use husky_entity_route::EntityRoutePtr;
use husky_visual_syntax::StaticVisualTy;
pub use lastx::*;

use super::*;
use husky_check_utils::should_eq;

pub trait __VecX<T> {
    fn ilen(&self) -> i32;

    fn __call__(__variadics: Vec<T>) -> Self;

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T>;

    fn popx(&mut self) -> T;

    fn firstx(&self) -> &T;

    fn firstx_mut(&mut self) -> &mut T;

    fn lastx(&self) -> &T;

    fn lastx_mut(&mut self) -> &mut T;
}

impl<T> __VecX<T> for Vec<T> {
    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn __call__(__variadics: Vec<T>) -> Self {
        __variadics
    }

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T> {
        CyclicSlice::<T> {
            start,
            end,
            total: self.as_slice(),
        }
    }

    fn popx(&mut self) -> T {
        self.pop().unwrap()
    }

    fn firstx(&self) -> &T {
        self.first().unwrap()
    }

    fn firstx_mut(&mut self) -> &mut T {
        self.first_mut().unwrap()
    }

    fn lastx(&self) -> &T {
        self.last().unwrap()
    }

    fn lastx_mut(&mut self) -> &mut T {
        self.last_mut().unwrap()
    }
}

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Vec",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        static_trait_impls: &[StaticTraitImplDefn {
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "E"),
                EntityStaticDefn {
                    dev_src: __static_dev_src!(),
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
                            copy_access: __SpecificRoutineFp(generic_vec_element_copy_access),
                            eval_ref_access: __SpecificRoutineFp(
                                generic_vec_element_eval_ref_access,
                            ),
                            temp_ref_access: __SpecificRoutineFp(
                                generic_vec_element_temp_ref_access,
                            ),
                            move_access: __SpecificRoutineFp(generic_vec_element_move_access),
                            temp_mut_access: __SpecificRoutineFp(
                                generic_vec_element_borrow_mut_access,
                            ),
                            nargs: 2,
                            dev_src: __static_dev_src!(),
                        })),
                    },
                },
            ],
            dev_src: __static_dev_src!(),
        }],
        ty_members: &[
            &VEC_LEN,
            &VEC_PUSH,
            &VEC_POPX,
            &VEC_FIRST,
            &VEC_LAST,
            &VEC_CYCLIC_SLICE,
        ],
        variants: &[],
        kind: TyKind::Vec,
        visual_ty: StaticVisualTy::Group,
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    items: &[],
    variant: EntityStaticDefnVariant::Function {
        spatial_parameters: &[],
        parameters: &[],
        variadic_template: StaticVariadicTemplate::SingleTyped { ty: "E" },
        output_ty: "Vec<E>",
        output_liason: OutputLiason::Transfer,
        linkage: generic_routine_linkage!(generic_vec_type_call).into(),
    },
    dev_src: __static_dev_src!(),
};

pub(crate) fn generic_vec_type_call<'temp, 'eval>(
    ty: EntityRoutePtr,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let mut data = vec![];
    for value in values {
        data.push(value.move_into_member())
    }
    __TempValue::OwnedEval(__OwnedValue::new(VirtualVec::new(ty, data)))
}

fn generic_vec_push<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let element = values[1].move_into_member();
    let generic_vec: &mut VirtualVec<'eval> = values[0].downcast_mut();
    generic_vec.push(element);
    __TempValue::Copyable(().into())
}

fn generic_vec_pop<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &mut VirtualVec<'eval> = values[0].downcast_mut();
    generic_vec.pop().unwrap().into_stack()
}

pub(crate) fn generic_vec_element_move_access<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    let i: usize = match values[1] {
        __TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].copy_into_stack()
}

pub(crate) fn generic_vec_element_eval_ref_access<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    let i: usize = match values[1] {
        __TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    let any_ptr: *const (dyn __AnyValueDyn<'eval> + 'eval) = this_value[i].any_ref();
    match values[0] {
        __TempValue::EvalRef(_) => __TempValue::EvalRef(__EvalRef(unsafe { &*any_ptr })),
        __TempValue::TempRefEval(_) => __TempValue::TempRefEval(unsafe { &*any_ptr }),
        _ => panic!(),
    }
}

pub(crate) fn generic_vec_element_temp_ref_access<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let this_value: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    let i: usize = match values[1] {
        __TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    this_value[i].bind_temp_ref()
}

pub(crate) fn generic_vec_element_borrow_mut_access<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let i: usize = match values[1] {
        __TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    let (this_value, stack_idx, gen): (&mut VirtualVec<'eval>, _, _) =
        values[0].downcast_mut_full();
    if i >= this_value.len() {
        todo!()
    }
    this_value[i].bind_mut(stack_idx)
}

pub static VEC_LEN: EntityStaticDefn = EntityStaticDefn {
    name: "ilen",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(generic_vec_len, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: __static_dev_src!(),
};

fn generic_vec_len<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    __TempValue::Copyable(len.into())
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
    name: "push",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::TempRefMut,
        parameters: &[StaticParameter {
            liason: ParameterLiason::Move,
            ty: "E",
            name: "element",
        }],
        output_ty: "void",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(generic_vec_push, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: __static_dev_src!(),
};

pub static VEC_POPX: EntityStaticDefn = EntityStaticDefn {
    name: "popx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::TempRefMut,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(generic_vec_pop, none)),
        output_liason: OutputLiason::Transfer,
    },
    dev_src: __static_dev_src!(),
};
