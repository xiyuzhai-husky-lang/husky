use super::*;
use dev_utils::__StaticDevSource;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct __MemberLinkage {
    pub copy_access: __SpecificRoutineFp,
    pub eval_ref_access: __SpecificRoutineFp,
    pub temp_ref_access: __SpecificRoutineFp,
    pub temp_mut_access: __SpecificRoutineFp,
    pub move_access: __SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: __StaticDevSource,
}

impl __MemberLinkage {
    pub fn bind(&self, binding: Binding) -> __SpecificRoutineLinkage {
        __SpecificRoutineLinkage {
            fp: match binding {
                Binding::EvalRef => self.eval_ref_access,
                Binding::TempRef => self.temp_ref_access,
                Binding::TempRefMut => self.temp_mut_access,
                Binding::Move => self.move_access,
                Binding::Copy => self.copy_access,
            },
            dev_src: self.dev_src,
            opt_raw_fp: None,
        }
    }
}

#[macro_export]
macro_rules! method_elem_linkage {
    ($Type: ty, $method_name: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: method_elem_copy_fp!($Type, $method_name),
            eval_ref_access: method_elem_eval_ref_fp!($Type, $method_name),
            temp_ref_access: method_elem_temp_ref_fp!($Type, $method_name),
            temp_mut_access: method_elem_temp_mut_fp!($Type, $method_name),
            move_access: method_elem_move_fp!($Type, $method_name),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
#[macro_export]
macro_rules! eager_field_linkage {
    ($Type: ty, $field: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: field_copy_fp!($Type, $field),
            eval_ref_access: field_eval_ref_fp!($Type, $field),
            temp_ref_access: field_temp_ref_fp!($Type, $field),
            temp_mut_access: field_temp_mut_invalid_fp!($Type, $field),
            move_access: field_move_fp!($Type, $field),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
#[macro_export]
macro_rules! lazy_field_linkage {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&__EvalContext<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            __TempValue::EvalRef(__EvalRef(this_value.$field(__opt_ctx.unwrap())))
        }
        specific_transfer_linkage!(__wrapper)
    }};
}

#[macro_export]
macro_rules! eager_mut_field_linkage {
    ($Type: ty, $field: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: field_copy_fp!($Type, $field),
            eval_ref_access: field_eval_ref_fp!($Type, $field),
            temp_ref_access: field_temp_ref_fp!($Type, $field),
            temp_mut_access: field_temp_mut_fp!($Type, $field),
            move_access: field_move_fp!($Type, $field),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! index_linkage {
    ($Type: ty) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: index_copy_fp!($Type),
            eval_ref_access: index_eval_ref_fp!($Type),
            temp_ref_access: index_temp_ref_fp!($Type),
            temp_mut_access: index_temp_mut_fp!($Type),
            move_access: index_move_fp!($Type),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
