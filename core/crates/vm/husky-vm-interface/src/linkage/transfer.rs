use super::*;

#[macro_export]
macro_rules! transfer_linkage {
    ($wrapper: expr, some $raw_fp: expr) => {{
        __Linkage::Transfer(linkage_fp!($wrapper, some $raw_fp))
    }};
    ($wrapper: expr, none) => {{
        __Linkage::Transfer(linkage_fp!($wrapper, none))
    }};
}

#[macro_export]
macro_rules! feature_linkage {
    ($f: expr, $return_ty_VTABLE: expr) => {{
        unsafe fn __wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            __arguments: &mut [__Register<'eval>],
        ) -> __Register<'eval> {
            __Register::new_eval_ref($f(__opt_ctx.unwrap()), &$return_ty_VTABLE)
        }
        __Linkage::Transfer(linkage_fp!(__wrapper, none))
    }};
}
