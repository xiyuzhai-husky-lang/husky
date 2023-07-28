#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! transfer_linkage {
    ($wrapper: expr, some base $raw_fp: expr) => {{
        __Linkage::Transfer(resolved_linkage!($wrapper, some base $raw_fp))
    }};
    ($wrapper: expr, some ctx $raw_fp: expr) => {{
        __Linkage::Transfer(resolved_linkage!($wrapper, some ctx $raw_fp))
    }};
    ($wrapper: expr, none) => {{
        __Linkage::Transfer(resolved_linkage!($wrapper, none))
    }};
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! feature_linkage {
    ($f: expr, $OUTPUT_TY: ty, $OUTPUT_TY_VTABLE: expr) => {{
        unsafe fn __wrapper(
            __arguments: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            __RegularValue::new_leash::<$OUTPUT_TY>($f(__opt_ctx.unwrap()), &$OUTPUT_TY_VTABLE)
        }
        __Linkage::Transfer(resolved_linkage!(__wrapper, none))
    }};
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! opt_feature_linkage {
    ($f: expr, $OUTPUT_TY: ty, $OUTPUT_TY_VTABLE: expr) => {{
        unsafe fn __wrapper(
            __arguments: &mut [__RegularValue],
            __opt_ctx: Option<&dyn __EvalContext>,
        ) -> __RegularValue {
            __RegularValue::new_opt_leash::<$OUTPUT_TY>($f(__opt_ctx.unwrap()), &$OUTPUT_TY_VTABLE)
        }
        __Linkage::Transfer(resolved_linkage!(__wrapper, none))
    }};
}
