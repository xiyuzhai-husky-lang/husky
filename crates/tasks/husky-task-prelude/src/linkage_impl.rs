use crate::DevEvalContext;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type BasePoint: Copy + 'static;
    type Value;
    type FnArguments: Default;
    type GnArguments;

    fn eval_fn(self, ctx: DevEvalContext<Self>, arguments: Self::FnArguments) -> Self::Value;
    fn eval_gn(self) -> Self::Value;
}
