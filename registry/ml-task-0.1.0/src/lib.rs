use ad_hoc_devsoul_dependency::{ugly::*, *};
use husky_core::*;

ad_hoc_devsoul_dependency::init_crate!();

pub trait IsMlTask {
    type Input;
}
