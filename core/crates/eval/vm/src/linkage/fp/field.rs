#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            value.$field.take_copyable().into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_eval_ref_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref();
            __EvalRef(value.$field.upcast_any()).into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
#[macro_export]
macro_rules! field_temp_ref_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            __TempValue::TempRefEval(unsafe { value.$field.upcast_arb_any() })
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_temp_mut_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            todo!("field_temp_mut_fp")
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_move_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            todo!("field_move_fp")
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
