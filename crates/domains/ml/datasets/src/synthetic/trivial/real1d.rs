use crate::{synthetic::SimpleSyntheticDataset, *};
use entity_route::StaticFuncSignature;
use std::sync::Arc;
use vm::{BoxedValue, Compiled, StackValue};
use xrng::XRng;

pub const SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[
        ("dataset1", DATASET1_SCOPE_DATA),
        ("dataset2", DATASET2_SCOPE_DATA),
    ],
    decl: BuiltinEntityDecl::Module,
};

pub const DATASET1_SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[],
    decl: BuiltinEntityDecl::Func(StaticFuncSignature {
        inputs: vec![],
        output: "Dataset<f32, i32>",
        compiled: Some(Compiled {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset1()))),
        }),
    }),
};

pub const DATASET2_SCOPE_DATA: &BuiltinEntityData = &BuiltinEntityData {
    subscopes: &[],
    decl: BuiltinEntityDecl::Func(StaticFuncSignature {
        inputs: vec![],
        output: "Dataset<f32, i32>",
        compiled: Some(Compiled {
            call: |_| Ok(StackValue::Boxed(BoxedValue::new(dataset2()))),
        }),
    }),
};

pub fn gen_sample1<'eval>(seed: u64, idx: usize) -> LabeledData<'eval> {
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

pub fn gen_sample2<'eval>(seed: u64, idx: usize) -> LabeledData<'eval> {
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

pub fn dataset1<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1223418012u64, gen_sample1))
}

pub fn dataset2<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1213148012u64, gen_sample2))
}
