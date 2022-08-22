use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct __MemberLinkage {
    pub copy_fp: __ResolvedLinkage,
    pub eval_ref_fp: __ResolvedLinkage,
    pub temp_ref_fp: __ResolvedLinkage,
    pub temp_mut_fp: __ResolvedLinkage,
    pub move_fp: __ResolvedLinkage,
}

#[cfg(feature = "binding")]
use husky_vm_binding::Binding;

#[cfg(feature = "binding")]
impl __MemberLinkage {
    pub fn bind(&self, binding: Binding) -> __ResolvedLinkage {
        match binding {
            Binding::EvalRef => self.eval_ref_fp,
            Binding::TempRef => self.temp_ref_fp,
            Binding::TempMut => self.temp_mut_fp,
            Binding::Move => self.move_fp,
            Binding::Copy => self.copy_fp,
            Binding::DerefCopy => self.copy_fp,
        }
    }
}

#[macro_export]
macro_rules! method_elem_linkage {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: method_elem_copy_fp!($Type, $TYPE_VTABLE, $ELEMENT_TYPE_VTABLE, $method_name),
            eval_ref_fp: method_elem_eval_ref_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            temp_ref_fp: method_elem_temp_ref_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            temp_mut_fp: method_elem_temp_mut_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            move_fp: method_elem_move_fp!($Type, $TYPE_VTABLE, $ELEMENT_TYPE_VTABLE, $method_name),
        })
    }};
}

#[macro_export]
macro_rules! lazy_field_linkage {
    ($Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_FIELD_TY: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        fn __wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            __Register::new_eval_ref::<$INTRINSIC_FIELD_TY>(
                this_value.$field(__opt_ctx.unwrap()),
                &$FIELD_TY_VTABLE,
            )
        }
        transfer_linkage!(__wrapper, none)
    }};
}

#[macro_export]
macro_rules! eager_field_linkage {
    (
        $liason: tt,
        $canonical_kind: tt,
        $reg_memory_kind: tt,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: field_copy_fp!(
                $canonical_kind,
                $reg_memory_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            eval_ref_fp: field_eval_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            temp_ref_fp: field_temp_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            temp_mut_fp: field_temp_mut_fp!(
                $liason,
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            move_fp: field_move_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
        })
    }};
}

#[macro_export]
macro_rules! index_linkage {
    (
        $liason: tt,
        $canonical_kind: tt,
        $reg_memory_kind: tt,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: index_copy_fp!(
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE,
                $canonical_kind,
                $reg_memory_kind
            ),
            eval_ref_fp: index_eval_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            temp_ref_fp: index_temp_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            temp_mut_fp: index_temp_mut_fp!(
                $liason,
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            move_fp: index_move_fp!(
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
        })
    }};
}
