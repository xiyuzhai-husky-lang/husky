#[macro_export]
macro_rules! field_copy_fp {
    (
        $canonical_kind: ident,
        $reg_memory_kind: tt,
        $Type: ty, $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register],
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            register_new_copyable!(
                $canonical_kind,
                $reg_memory_kind,
                value.$field,
                $INTRINSIC_FIELD_TY,
                $FIELD_TY_VTABLE
            )
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_eval_ref_fp {
    (
        Intrinsic,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            __Register::new_eval_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}
#[macro_export]
macro_rules! field_temp_ref_fp {
    (
        Intrinsic,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_temp_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_temp_mut_fp {
    (
        mutable,
        Intrinsic,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            let value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_temp_mut::<$INTRINSIC_FIELD_TY>(&mut value.$field, &$FIELD_TY_VTABLE)
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
    (
        immutable,
        Intrinsic,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            panic!("field_temp_mut_invalid_fp")
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! field_move_fp {
    (
        Intrinsic,
        $Type: ty, $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        fn wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __LinkageFp {
            wrapper,
            opt_fp: None,
            dev_src: static_dev_src!(),
        }
    }};
}
