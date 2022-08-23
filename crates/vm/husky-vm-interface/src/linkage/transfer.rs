use super::*;

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
        unsafe fn __wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            __arguments: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            __Register::new_eval_ref::<$OUTPUT_TY>($f(__opt_ctx.unwrap()), &$OUTPUT_TY_VTABLE)
        }
        __Linkage::Transfer(resolved_linkage!(__wrapper, none))
    }};
}

#[cfg(feature = "linkage_macro")]
#[macro_export]
macro_rules! opt_feature_linkage {
    ($f: expr, $OUTPUT_TY: ty, $OUTPUT_TY_VTABLE: expr) => {{
        unsafe fn __wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            __arguments: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            __Register::new_opt_eval_ref::<$OUTPUT_TY>($f(__opt_ctx.unwrap()), &$OUTPUT_TY_VTABLE)
        }
        __Linkage::Transfer(resolved_linkage!(__wrapper, none))
    }};
}
