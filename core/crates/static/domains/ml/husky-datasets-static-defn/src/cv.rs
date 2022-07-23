pub mod imagenet;
pub mod mnist;

use crate::*;
use mnist::*;
use std::sync::Arc;

pub static CV_MOD_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "cv",
    items: &[MNIST_SCOPE_DATA],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::__static_dev_src!(),
};
