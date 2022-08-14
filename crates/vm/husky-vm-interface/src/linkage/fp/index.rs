#[macro_export]
macro_rules! index_copy_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $copy_kind: tt) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            println!("here");
            let index_value: usize = values[1].downcast_i32() as usize;
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            register_new_copyable!(this_value[index_value], $ELEMENT_TYPE_VTABLE, $copy_kind)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_eval_ref_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            println!("here");
            let this_value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_eval_ref(&this_value[index_value], &$ELEMENT_TYPE_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_temp_ref_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            println!("here");
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_temp_ref(&this_value[index_value], &$ELEMENT_TYPE_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_move_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr) => {{
        __LinkageFp {
            wrapper: |_, values| -> __Register { todo!("move") },
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_temp_mut_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, mutable) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            println!("here index_temp_mut_fp");
            let index_value: usize = values[1].downcast_i32() as usize;
            let this_value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_temp_mut(&mut this_value[index_value], &$ELEMENT_TYPE_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, immutable) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            println!("here index_temp_mut_fp");
            panic!("can't mutate immutable")
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}
