mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use mnist::*;
use static_decl::StaticFuncDecl;
use std::sync::Arc;
use vm::{BoxedValue, RoutineLinkage, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &StaticEntityDefn = &StaticEntityDefn {
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    decl: StaticEntityDecl::Module,
};
