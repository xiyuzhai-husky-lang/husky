use std::ops::Range;

use husky_vm::{GenericArgument, __VMResult, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use ordered_float::NotNan;

pub struct FlagVectorField {
    valuess: Vec<Vec<NotNan<f32>>>,
    flags: Vec<bool>,
    label0: i32,
}

impl FlagVectorField {
    pub fn from_registers(
        label0: &GenericArgument,
        arguments: &[GenericArgument],
        labels: &[i32],
    ) -> __VMResult<Self> {
        let label0: &__VirtualEnum = label0.value().downcast_temp_ref(&__VIRTUAL_ENUM_VTABLE);
        let label0 = label0.kind_idx;
        Ok(Self {
            valuess: arguments
                .iter()
                .map(|value| {
                    value
                        .values()
                        .iter()
                        .map(|r| {
                            let val = r.downcast_f32();
                            NotNan::new(val).expect("todo")
                        })
                        .collect()
                })
                .collect(),
            flags: labels.iter().map(|label| *label == label0).collect(),
            label0,
        })
    }

    pub fn flag_ranges(&self) -> Vec<FlagRange> {
        self.valuess
            .iter()
            .map(|values| FlagRange::from_values(values, &self.flags))
            .collect()
    }

    pub fn label0(&self) -> i32 {
        self.label0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FlagRange {
    true_range: ClosedRange<NotNan<f32>>,
    false_range: ClosedRange<NotNan<f32>>,
}

impl FlagRange {
    pub fn apply(&self, v: NotNan<f32>) -> FlagRangeApplyResult {
        FlagRangeApplyResult {
            within_true_range: self.true_range.is_within(v),
            within_false_range: self.false_range.is_within(v),
        }
    }

    pub fn from_values(values: &[NotNan<f32>], flags: &[bool]) -> Self {
        Self {
            true_range: ClosedRange::from_values(
                std::iter::zip(values.iter(), flags.iter()).filter_map(|(value, flag)| {
                    if *flag {
                        Some(*value)
                    } else {
                        None
                    }
                }),
            ),
            false_range: ClosedRange::from_values(
                std::iter::zip(values.iter(), flags.iter()).filter_map(|(value, flag)| {
                    if !*flag {
                        Some(*value)
                    } else {
                        None
                    }
                }),
            ),
        }
    }
}

pub struct FlagRangeApplyResult {
    within_true_range: bool,
    within_false_range: bool,
}

impl FlagRangeApplyResult {
    pub fn within_true_range(&self) -> bool {
        self.within_true_range
    }
    pub fn within_false_range(&self) -> bool {
        self.within_false_range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClosedRange<T>
where
    T: PartialOrd + Ord,
{
    pub start: T,
    pub end: T,
}

impl<T> ClosedRange<T>
where
    T: PartialOrd + Ord,
{
    fn from_values(iter: impl Iterator<Item = T> + Clone) -> Self {
        let start = iter.clone().min().unwrap();
        let end = iter.max().unwrap();
        Self { start, end }
    }

    fn is_within(&self, v: T) -> bool {
        self.start <= v && v <= self.end
    }
}
