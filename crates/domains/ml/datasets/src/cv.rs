mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use mnist::*;
use static_defn::StaticCallDefn;
use std::sync::Arc;
use vm::{BoxedValue, Linkage, StackValue};

pub static CV_MOD_DEFN: &StaticEntityDefn = &StaticEntityDefn {
    name: "cv",
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    variant: StaticEntityDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};
