pub mod label;

use ad_hoc_devsoul_dependency::{ugly::*, *};
use husky_core::*;

pub trait IsMlTask<VarId>
where
    VarId: std::fmt::Debug + Copy + Eq + 'static,
{
    type Input;

    type INPUT: __IsStaticVar<VarId>;
}
