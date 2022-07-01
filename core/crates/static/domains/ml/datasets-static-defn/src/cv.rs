pub mod imagenet;
pub mod mnist;

use crate::*;
use mnist::*;
use std::sync::Arc;

pub static CV_MOD_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "cv",
    items: &[MNIST_SCOPE_DATA],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
