#[macro_export]
macro_rules! method_elem_copy_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        fn wrapper(
            values: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            let this_value: &$Type = unsafe { values[0].downcast_temp_ref(&$TYPE_VTABLE) };
            todo!()
        }
        __ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
        }
    }};
}

#[macro_export]
macro_rules! method_elem_leash_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        fn wrapper(
            values: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            let this_value: &'static $Type = values[0].downcast_leash(&$TYPE_VTABLE);
            __RegularValue::new_leash(this_value.$method_name(), &$ELEMENT_TYPE_VTABLE)
        }
        __ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
        }
    }};
}

#[macro_export]
macro_rules! method_elem_temp_ref_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        fn wrapper(
            values: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            let this_value: &$Type = unsafe { values[0].downcast_temp_ref(&$TYPE_VTABLE) };
            unsafe {
                __RegularValue::new_temp_ref(this_value.$method_name(), &$ELEMENT_TYPE_VTABLE)
            }
        }
        __ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
        }
    }};
}

#[macro_export]
macro_rules! method_elem_move_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        __ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper: |values, _| -> __RegularValue { todo!("move") },
            opt_thick_fp: __OptVirtualThickFp::none(),
        }
    }};
}

#[macro_export]
macro_rules! method_elem_temp_mut_fp {
    ($Type: ty, $TYPE_VTABLE: expr, $ELEMENT_TYPE_VTABLE: expr, $method_name: ident) => {{
        unsafe fn wrapper(
            values: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            let this_value: &mut $Type = values[0].downcast_temp_mut(&$TYPE_VTABLE);
            todo!()
            // Ok(__TempValue::TempRefMutEval {
            //     value: &mut this_value[method_elem_value],
            //     owner,
            //     gen: (),
            // })
        }
        __ResolvedLinkage {
            dev_src: static_dev_src!(),
            wrapper,
            opt_thick_fp: __OptVirtualThickFp::none(),
        }
    }};
}
