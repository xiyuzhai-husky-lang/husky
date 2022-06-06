use crate::*;
use compile_time_db::AskCompileTime;
use upcast::Upcast;
use vm::InterpreterQueryGroup;

pub trait FeatureEvalQueryGroup:
    InterpreterQueryGroup + Upcast<dyn InterpreterQueryGroup> + AskCompileTime
{
}
