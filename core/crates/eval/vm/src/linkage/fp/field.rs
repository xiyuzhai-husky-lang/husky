#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $field: ident) => {{
        fn field_copy_access<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __EvalResult<__TempValue<'temp, 'eval>> {
            let value: &$Type = values[0].downcast_temp_ref();
            Ok(value.$field.take_copyable_dyn().into())
        }
        __SpecificRoutineFp(field_copy_access)
    }};
}

#[macro_export]
macro_rules! field_linkage {
    ($Type: ty, $field: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: field_copy_fp!($Type, $field),
            eval_ref_access: __SpecificRoutineFp(|values| todo!()),
            temp_ref_access: __SpecificRoutineFp(|values| todo!()),
            temp_mut_access: __SpecificRoutineFp(|values| todo!()),
            move_access: __SpecificRoutineFp(|values| todo!()),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
