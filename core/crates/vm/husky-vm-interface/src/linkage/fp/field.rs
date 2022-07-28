#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident, $copy_kind: tt) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register],
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            register_new_copyable!(value.$field, $FieldTy, $copy_kind)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_eval_ref_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref();
            __Register::new_eval_ref(&value.$field, &$FIELD_TY_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}
#[macro_export]
macro_rules! field_temp_ref_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            __Register::new_temp_ref(&value.$field, &$FIELD_TY_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_temp_mut_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let (value, stack_idx, gen): (&mut $Type, _, _) = values[0].downcast_temp_mut();
            __Register::new_temp_mut(&mut value.$field)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_temp_mut_invalid_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            panic!("field_temp_mut_invalid_fp")
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_move_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $FieldTy: ty, $FIELD_TY_VTABLE: expr, $field: ident) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: husky_dev_utils::static_dev_src!(),
        }
    }};
}
