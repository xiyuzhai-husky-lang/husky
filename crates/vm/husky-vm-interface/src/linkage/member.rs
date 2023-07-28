use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct __MemberLinkageGroup {
    pub copy_resolved_linkage: __ResolvedLinkage,
    pub leash_resolved_linkage: __ResolvedLinkage,
    pub temp_ref_resolved_linkage: __ResolvedLinkage,
    pub temp_mut_resolved_linkage: __ResolvedLinkage,
    pub move_resolved_linkage: __ResolvedLinkage,
}

#[cfg(feature = "binding")]
use husky_vm_binding::Binding;

#[cfg(feature = "binding")]
impl __MemberLinkageGroup {
    pub fn bind(&self, binding: Binding) -> __ResolvedLinkage {
        match binding {
            Binding::Leash => self.leash_resolved_linkage,
            Binding::TempRef => self.temp_ref_resolved_linkage,
            Binding::TempMut => self.temp_mut_resolved_linkage,
            Binding::Move => self.move_resolved_linkage,
            Binding::Copy => self.copy_resolved_linkage,
            Binding::DerefCopy => self.copy_resolved_linkage,
        }
    }
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! method_elem_linkage {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        __LinkageGroup::Member(&__MemberLinkageGroup {
            copy_resolved_linkage: method_elem_copy_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            leash_resolved_linkage: method_elem_leash_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            temp_ref_resolved_linkage: method_elem_temp_ref_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            temp_mut_resolved_linkage: method_elem_temp_mut_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
            move_resolved_linkage: method_elem_move_fp!(
                $Type,
                $TYPE_VTABLE,
                $ELEMENT_TYPE_VTABLE,
                $method_name
            ),
        })
    }};
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! lazy_field_linkage {
    ($Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_FIELD_TY: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        fn __wrapper(
            values: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            let this_value: &'static $Type = values[0].downcast_leash(&$TYPE_VTABLE);
            __RegularValue::new_leash::<$INTRINSIC_FIELD_TY>(
                this_value.$field(__opt_ctx.unwrap()),
                &$FIELD_TY_VTABLE,
            )
        }
        transfer_linkage!(__wrapper, none)
    }};
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! eager_field_linkage {
    (
        $contract: tt,
        $canonical_kind: tt,
        $reg_memory_kind: tt,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        __LinkageGroup::Member(&__MemberLinkageGroup {
            copy_resolved_linkage: field_copy_fp!(
                $canonical_kind,
                $reg_memory_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            leash_resolved_linkage: field_leash_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            temp_ref_resolved_linkage: field_temp_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            temp_mut_resolved_linkage: field_temp_mut_fp!(
                $contract,
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE,
                $field
            ),
            move_resolved_linkage: field_move_fp!(
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
        $contract: tt,
        $canonical_kind: tt,
        $reg_memory_kind: tt,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        __LinkageGroup::Member(&__MemberLinkageGroup {
            copy_resolved_linkage: index_copy_fp!(
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE,
                $canonical_kind,
                $reg_memory_kind
            ),
            leash_resolved_linkage: index_leash_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            temp_ref_resolved_linkage: index_temp_ref_fp!(
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            temp_mut_resolved_linkage: index_temp_mut_fp!(
                $contract,
                $canonical_kind,
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
            move_resolved_linkage: index_move_fp!(
                $Type,
                $TYPE_VTABLE,
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            ),
        })
    }};
}
