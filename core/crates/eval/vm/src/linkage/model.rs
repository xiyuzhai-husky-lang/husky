use super::*;

pub struct ModelLinkage {
    pub train: fn(&dyn std::any::Any) -> EvalResult,
    pub eval: for<'eval> fn(&EvalValue<'static>, Vec<EvalValue<'eval>>) -> EvalValueResult<'eval>, //ugly
}

impl PartialEq for ModelLinkage {
    fn eq(&self, other: &Self) -> bool {
        (self.train as usize) == (other.train as usize)
            && (self.eval as usize) == (other.eval as usize)
    }
}

impl Eq for ModelLinkage {}

impl std::fmt::Debug for ModelLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
