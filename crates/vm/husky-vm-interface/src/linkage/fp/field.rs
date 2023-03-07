#[cfg(feature = "linkage_macro")]
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
            values: &mut [__Register],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
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
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[cfg(feature = "linkage_macro")]
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
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            __Register::new_eval_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Optional,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            __Register::new_opt_eval_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Leash,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_eval_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        OptionalLeash,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_opt_eval_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[cfg(feature = "linkage_macro")]
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
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_temp_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Leash,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_temp_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Optional,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_opt_temp_ref::<$INTRINSIC_FIELD_TY>(
                &value.$field.as_ref(),
                &$FIELD_TY_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        OptionalLeash,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            __Register::new_opt_temp_ref::<$INTRINSIC_FIELD_TY>(&value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[cfg(feature = "linkage_macro")]
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
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_temp_mut::<$INTRINSIC_FIELD_TY>(&mut value.$field, &$FIELD_TY_VTABLE)
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        mutable,
        Optional,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_opt_temp_mut::<$INTRINSIC_FIELD_TY>(
                value.$field.as_mut(),
                &$FIELD_TY_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        mutable,
        Leash,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            panic!("invalid")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        mutable,
        OptionalRef,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            panic!("invalid")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (immutable, $($args: tt),*) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            panic!("field_temp_mut_invalid_fp")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[cfg(feature = "linkage_macro")]
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
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Optional,
        $Type: ty, $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        Leash,
        $Type: ty, $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (
        OptionalLeash,
        $Type: ty, $TYPE_VTABLE: expr,
        $INTRINSIC_FIELD_TY: ty,
        $FIELD_TY_VTABLE: expr,
        $field: ident
    ) => {{
        fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            todo!("field_move_fp")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}
