#[macro_export]
macro_rules! index_copy_fp {
    (
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr,
        $canonical_kind: ident,
        $reg_memory_kind: tt
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let index_value: usize = values[1].downcast_i32() as usize;
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            register_new_copyable!(
                $canonical_kind,
                $reg_memory_kind,
                this_value[index_value],
                $INTRINSIC_ELEMENT_TY,
                $ELEMENT_TYPE_VTABLE
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_eval_ref_fp {
    (Intrinsic, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_eval_ref::<$INTRINSIC_ELEMENT_TY>(
                &this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (Optional, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &'eval $Type = values[0].downcast_eval_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_opt_eval_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value].as_ref(),
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (Leash, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_eval_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (OptionalLeash, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_opt_eval_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_temp_ref_fp {
    (
        Intrinsic,
        $Type: ty,
        $TYPE_VTABLE: expr,
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_temp_ref::<$INTRINSIC_ELEMENT_TY>(
                &this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
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
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_opt_temp_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value].as_ref(),
                &$ELEMENT_TYPE_VTABLE,
            )
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
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_temp_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
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
        $INTRINSIC_ELEMENT_TY: ty,
        $ELEMENT_TYPE_VTABLE: expr
    ) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let this_value: &$Type = values[0].downcast_temp_ref(&$TYPE_VTABLE);
            let index_value: usize = values[1].downcast_i32() as usize;
            __Register::new_opt_temp_ref::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_move_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        __ResolvedLinkage {
            wrapper: |values, _| -> __Register { todo!("move") },
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! index_temp_mut_fp {
    (mutable, Intrinsic, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let index_value: usize = values[1].downcast_i32() as usize;
            let this_value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_temp_mut::<$INTRINSIC_ELEMENT_TY>(
                &mut this_value[index_value],
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (mutable, Optional, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
        unsafe fn wrapper<'eval>(
            values: &mut [__Register<'eval>],
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
        ) -> __Register<'eval> {
            let index_value: usize = values[1].downcast_i32() as usize;
            let this_value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            __Register::new_opt_temp_mut::<$INTRINSIC_ELEMENT_TY>(
                this_value[index_value].as_mut(),
                &$ELEMENT_TYPE_VTABLE,
            )
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
    (mutable, Leash, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
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
    (mutable, OptionalLeash, $Type: ty, $TYPE_VTABLE: expr, $INTRINSIC_ELEMENT_TY: ty, $ELEMENT_TYPE_VTABLE: expr) => {{
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
            panic!("can't mutate immutable")
        }
        __ResolvedLinkage {
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
            dev_src: static_dev_src!(),
        }
    }};
}
