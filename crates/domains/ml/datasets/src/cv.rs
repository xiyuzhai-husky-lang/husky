mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use crate::{synthetic::SimpleSyntheticDataset, *};
use entity_route::StaticFuncSignature;
use mnist::*;
use std::sync::Arc;
use vm::{BoxedValue, Compiled, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    signature: BuiltinScopeSignature::Module,
};
