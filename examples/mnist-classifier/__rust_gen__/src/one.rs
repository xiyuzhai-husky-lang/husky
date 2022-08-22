use crate::*;

pub(crate) fn downmost<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
) -> Option<f32> {
    return Some(cc.displacement(__ctx).y);
}
