use ad_hoc_devsoul_dependency::{ugly::*, *};
use husky_core::*;

pub trait IsMlTask<StaticVarId>
where
    StaticVarId: std::fmt::Debug + Copy + Eq,
{
    type Input;

    type INPUT: __IsStaticVar<StaticVarId>;
}
