use crate::{synthetic::SimpleSyntheticDataset, *};
use entity_kind::RoutineKind;
use std::sync::Arc;
use vm::{linkage, Linkage, OutputLiason, OwnedValue, TempValue};
use xrng::XRng;

pub const REAL_1D_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "real1d",
    subscopes: &[
        ("dataset1", DATASET1_SCOPE_DATA),
        ("dataset2", DATASET2_SCOPE_DATA),
    ],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub const DATASET1_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset1",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: vec![],
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: linkage!(|_| Ok(TempValue::EvalOwned(OwnedValue::new(dataset1()))), 0),
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub const DATASET2_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset2",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: vec![],
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: linkage!(|_| Ok(TempValue::EvalOwned(OwnedValue::new(dataset2()))), 0),
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
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
