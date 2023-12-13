pub mod any;

pub trait IsLinkageImpl: Send + Copy {
    type Value;
    fn eval_fn() -> Self::Value;
    fn eval_gn() -> Self::Value;
}
