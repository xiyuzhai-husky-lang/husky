mod imagenet;
mod mnist;

use crate::{labeled::LabeledData, *};
use crate::{synthetic::SimpleSyntheticDataset, *};
use mnist::*;
use scope::StaticFuncSignature;
use std::sync::Arc;
use vm::{BoxedValue, Compiled, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[("mnist", MNIST_SCOPE_DATA)],
    signature: StaticScopeSignature::Module,
};
