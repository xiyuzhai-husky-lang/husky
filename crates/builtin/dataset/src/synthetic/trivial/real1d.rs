pub const SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Module,
    subscopes: &[
        ("dataset1", DATASET1_SCOPE_DATA),
        ("dataset2", DATASET2_SCOPE_DATA),
    ],
    signature: StaticScopeSignature::Module,
};

pub const DATASET1_SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Func,
    subscopes: &[],
    signature: StaticScopeSignature::Func(StaticFuncSignature {
        inputs: vec![],
        output: "Dataset<f32>",
        compiled: Some(Compiled {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset1()))),
        }),
    }),
};

pub const DATASET2_SCOPE_DATA: &BuiltinScopeData = &BuiltinScopeData {
    scope_kind: ScopeKind::Func,
    subscopes: &[],
    signature: StaticScopeSignature::Func(StaticFuncSignature {
        inputs: vec![],
        output: "Dataset<f32>",
        compiled: Some(Compiled {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset2()))),
        }),
    }),
};

use std::sync::Arc;

use scope::StaticFuncSignature;
use vm::{BoxedValue, Compiled, StackValue};
use xrng::XRng;

use crate::{synthetic::SimpleSyntheticDataset, *};

pub fn gen_sample1(seed: u64, idx: usize) -> LabeledData {
    let mut xrng = XRng::new(((seed + (idx as u64)) >> 32) & ((idx as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: Arc::new(1.0f32),
            label: 1,
        }
    } else {
        LabeledData {
            input: Arc::new(-1.0f32),
            label: 1,
        }
    }
}

pub fn gen_sample2(seed: u64, idx: usize) -> LabeledData {
    let mut xrng = XRng::new(((seed + (idx as u64)) >> 32) & ((idx as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: Arc::new(1.0f32),
            label: 1,
        }
    } else {
        LabeledData {
            input: Arc::new(-1.0f32),
            label: 1,
        }
    }
}

pub fn dataset1() -> Dataset {
    Dataset::new(SimpleSyntheticDataset::new(1223418012u64, gen_sample1))
}

pub fn dataset2() -> Dataset {
    Dataset::new(SimpleSyntheticDataset::new(1213148012u64, gen_sample2))
}
