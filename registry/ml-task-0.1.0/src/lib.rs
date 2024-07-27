use ad_hoc_devsoul_dependency::{ugly::*, *};
use husky_core::*;

pub trait IsMlTask<StaticVarId> {
    type Input;

    type INPUT: __IsStaticVar<StaticVarId>;
}
