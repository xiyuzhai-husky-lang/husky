mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use mnist::*;
use static_decl::StaticCallDecl;
use std::sync::Arc;
use vm::{BoxedValue, Linkage, StackValue};
use xrng::XRng;

pub static SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    decl: StaticEntityDecl::Module,
};
