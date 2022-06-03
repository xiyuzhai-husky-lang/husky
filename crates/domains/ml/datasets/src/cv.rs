mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use mnist::*;
use std::sync::Arc;
use vm::{Linkage, OwnedValue, TempValue};

pub static CV_MOD_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "cv",
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
