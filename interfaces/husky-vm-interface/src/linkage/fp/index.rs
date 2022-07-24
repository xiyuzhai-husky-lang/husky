#[macro_export]
macro_rules! index_copy_fp {
    ($Type: ty, $ElementType: ty, $copy_kind: tt) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let index_value: usize = values[1].downcast::<i32>() as usize;
            let this_value: &$Type = values[0].downcast_temp_ref();
            register_new_copyable!(this_value[index_value], $ElementType, $copy_kind)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! index_eval_ref_fp {
    ($Type: ty) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            let index_value: usize = values[1].downcast::<i32>() as usize;
            __Register::new_eval_ref(&this_value[index_value])
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! index_temp_ref_fp {
    ($Type: ty) => {{
        __LinkageFp {
            wrapper: |_, values| -> __Register { todo!("temp ref") },
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! index_move_fp {
    ($Type: ty) => {{
        __LinkageFp {
            wrapper: |_, values| -> __Register { todo!("move") },
            opt_fp: None,
        }
    }};
}

#[macro_export]
macro_rules! index_temp_mut_fp {
    ($Type: ty) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let index_value: usize = values[1].downcast::<i32>() as usize;
            let this_value: &mut $Type = values[0].downcast_temp_mut();
            __Register::new_temp_mut(&mut this_value[index_value])
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
        }
    }};
}
