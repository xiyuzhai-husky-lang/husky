mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use crate::{synthetic::SimpleSyntheticDataset, *};
use entity_route::StaticFuncDecl;
use mnist::*;
use std::sync::Arc;
use vm::{BoxedValue, CompiledRustCall, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    decl: BuiltinEntityDecl::Module,
};
