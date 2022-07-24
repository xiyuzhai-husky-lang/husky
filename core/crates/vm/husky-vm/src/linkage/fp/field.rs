#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            value.$field.__take_copyable().into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_eval_ref_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref();
            __EvalRef(value.$field.__upcast_any()).into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
#[macro_export]
macro_rules! field_temp_ref_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            __TempValue::TempRefEval(unsafe { value.$field.__upcast_arb_any_ref() })
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_temp_mut_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let (value, stack_idx, gen): (&mut $Type, _, _) = values[0].downcast_mut_full();
            __TempValue::TempRefMutEval {
                value: unsafe { value.$field.__upcast_arb_any_mut() },
                owner: stack_idx,
                gen: (),
            }
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_temp_mut_invalid_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            panic!("field_temp_mut_invalid_fp")
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! field_move_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            todo!("field_move_fp")
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
