#[macro_export]
macro_rules! method_elem_copy_fp {
    ($Type: ty, $method_name: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref();
            todo!()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! method_elem_eval_ref_fp {
    ($Type: ty, $method_name: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            __TempValue::EvalRef(__EvalRef(this_value.$method_name()))
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! method_elem_temp_ref_fp {
    ($Type: ty, $method_name: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref();
            __TempValue::TempRefEval(unsafe { this_value.$method_name().__upcast_arb_any_ref() })
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! method_elem_move_fp {
    ($Type: ty, $method_name: ident) => {{
        __SpecificRoutineFp(|values| -> __TempValue { todo!("move") })
    }};
}

#[macro_export]
macro_rules! method_elem_temp_mut_fp {
    ($Type: ty, $method_name: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let (this_value, owner, _): (&mut $Type, _, _) = values[0].downcast_mut_full();
            todo!()
            // Ok(__TempValue::TempRefMutEval {
            //     value: &mut this_value[method_elem_value],
            //     owner,
            //     gen: (),
            // })
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
