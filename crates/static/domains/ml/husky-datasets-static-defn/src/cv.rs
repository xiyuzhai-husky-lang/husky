pub mod imagenet;
pub mod mnist;

use crate::*;
use imagenet::*;
use mnist::*;
use std::sync::Arc;

pub static CV_MOD_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "cv",
    items: &[MNIST_MOD, &IMAGENET_MOD],
    variant: EntityStaticDefnVariant::Module,
    dev_src: husky_dev_utils::static_dev_src!(),
};
