#[macro_export]
macro_rules! index_copy_fp {
    ($Type: ty) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref();
            let index_value: usize = values[1]
                .take_copyable()
                .take_i32()
                .try_into()
                .expect("todo");
            this_value[index_value].take_copyable_dyn().into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! index_eval_ref_fp {
    ($Type: ty) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            let index_value: usize = values[1]
                .take_copyable()
                .take_i32()
                .try_into()
                .expect("todo");
            Ok(__EvalRef(&this_value[index_value]).into())
        }
        __SpecificRoutineFp(__wrapper)
    }};
}

#[macro_export]
macro_rules! index_temp_ref_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> __TempValue { todo!("temp ref") })
    }};
}

#[macro_export]
macro_rules! index_move_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> __TempValue { todo!("move") })
    }};
}

#[macro_export]
macro_rules! index_temp_mut_fp {
    ($Type: ty) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let index_value: usize = values[1]
                .take_copyable()
                .take_i32()
                .try_into()
                .expect("todo");
            let (this_value, owner, _): (&mut $Type, _, _) = values[0].downcast_mut_full();
            __TempValue::TempRefMutEval {
                value: &mut this_value[index_value],
                owner,
                gen: (),
            }
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
