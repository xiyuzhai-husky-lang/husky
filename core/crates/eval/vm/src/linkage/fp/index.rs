#[macro_export]
macro_rules! index_copy_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> EvalResult<TempValue> {
            let this_value: &$Type = values[0].downcast_ref();
            let index_value: usize = values[1]
                .take_copyable()
                .take_i32()
                .try_into()
                .expect("todo");
            Ok(TempValue::Copyable(this_value[index_value].into()))
        })
    }};
}

#[macro_export]
macro_rules! index_eval_ref_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> EvalResult<TempValue> { todo!() })
    }};
}

#[macro_export]
macro_rules! index_temp_ref_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> EvalResult<TempValue> { todo!() })
    }};
}

#[macro_export]
macro_rules! index_move_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> EvalResult<TempValue> { todo!() })
    }};
}

#[macro_export]
macro_rules! index_temp_mut_fp {
    ($Type: ty) => {{
        __SpecificRoutineFp(|values| -> EvalResult<TempValue> {
            let index_value: usize = values[1]
                .take_copyable()
                .take_i32()
                .try_into()
                .expect("todo");
            let (this_value, owner, _): (&mut $Type, _, _) = values[0].downcast_mut_full();
            Ok(TempValue::TempRefMutEval {
                value: &mut this_value[index_value],
                owner,
                gen: (),
            })
        })
    }};
}
