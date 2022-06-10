use crate::*;
use husky_compile_time::AskCompileTime;
use upcast::Upcast;
use vm::InterpreterQueryGroup;

pub trait FeatureEvalQueryGroup:
    InterpreterQueryGroup + Upcast<dyn InterpreterQueryGroup> + AskCompileTime
{
}
