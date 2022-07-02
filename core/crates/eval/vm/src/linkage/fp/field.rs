#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $field: ident) => {{
        fn field_copy_access<'temp, 'eval>(
            values: &mut [TempValue<'temp, 'eval>],
        ) -> EvalResult<TempValue<'temp, 'eval>> {
            let value: &$Type = values[0].downcast_ref();
            Ok(value.$field.take_copyable_dyn().into())
        }
        SpecificRoutineFp(field_copy_access)
    }};
}

#[macro_export]
macro_rules! field_linkage {
    ($Type: ty, $field: ident) => {{
        Linkage::Member(&MemberLinkage {
            copy_access: field_copy_fp!($Type, $field),
            eval_ref_access: SpecificRoutineFp(|values| todo!()),
            temp_ref_access: SpecificRoutineFp(|values| todo!()),
            temp_mut_access: SpecificRoutineFp(|values| todo!()),
            move_access: SpecificRoutineFp(|values| todo!()),
            nargs: 1,
            dev_src: static_dev_src!(),
        })
    }};
}
