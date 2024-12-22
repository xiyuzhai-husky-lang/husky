use crate::*;
use usage_cap::error::{LlmCapResult, UsageCapError};

pub(crate) fn try_call_openai<R>(
    min_usage: usize,
    f: impl FnOnce() -> (usize, R),
) -> LlmCapResult<Result<R, (R, UsageCapError)>> {
    lazy_static! {
        static ref GLOBAL_CAP: UsageCap = UsageCap::new("OpenAI", "ENABLE_OPENAI_API_CALLING", 2);
    }

    GLOBAL_CAP.try_use(min_usage, f)
}
