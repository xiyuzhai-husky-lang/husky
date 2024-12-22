use crate::*;
use lazy_static::lazy_static;
use usage_cap::{
    error::{LlmCapResult, UsageCapError},
    UsageCap,
};

pub const POST_CALL_USAGE_MULTIPLIER: usize = 2;

pub(crate) fn try_call_gemini<R>(
    min_usage: usize,
    f: impl FnOnce() -> (usize, R),
) -> LlmCapResult<Result<R, (R, UsageCapError)>> {
    lazy_static! {
        static ref GLOBAL_CAP: UsageCap = UsageCap::new(
            "Gemini",
            "ENABLE_GEMINI_API_CALLING",
            POST_CALL_USAGE_MULTIPLIER
        );
    }

    GLOBAL_CAP.try_use(min_usage, f)
}
