use crate::*;
use llm_cap::error::{LlmCapError, LlmCapResult};

pub(crate) fn try_call_llm<R>(
    min_usage: usize,
    f: impl FnOnce() -> (usize, R),
) -> LlmCapResult<Result<R, (R, LlmCapError)>> {
    lazy_static! {
        static ref GLOBAL_CAP: LlmCap = LlmCap::new("OpenAI", "ENABLE_OPENAI_API_CALLING");
    }

    GLOBAL_CAP.try_call_llm(min_usage, f)
}
