use super::*;

pub struct FlagVectorField<Label> {
    valuess: Vec<Vec<NotNan<f32>>>,
    flags: Vec<bool>,
    label0: Label,
}

impl<Label> FlagVectorField<Label>
where
    Label: PartialEq + Eq + Copy,
{
    pub fn from_features(arguments: &[__ValReprInterface], label0: Label) -> Result<Self, ()> {
        todo!()
        // Ok(Self {
        //     valuess: arguments
        //         .iter()
        //         .map(|value| {
        //             value
        //                 .values()
        //                 .iter()
        //                 .map(|r| {
        //                     let val = r.downcast_f32();
        //                     NotNan::new(val).expect("todo")
        //                 })
        //                 .collect()
        //         })
        //         .collect(),
        //     flags: labels.iter().map(|label| *label == label0).collect(),
        //     label0,
        // })
    }

    fn raw_flag_ranges(&self) -> Option<Vec<FlagRange>> {
        self.valuess
            .iter()
            .map(|values| FlagRange::from_values(values.iter().copied(), &self.flags))
            .collect()
    }

    pub fn flag_ranges(&self, ntrim: i32, border_expand_rate: f32) -> Option<Vec<FlagRange>> {
        let raw_flag_ranges = self.raw_flag_ranges()?;
        Some(
            raw_flag_ranges
                .iter()
                .enumerate()
                .map(|(idx, raw_flag_range)| {
                    self.flag_range(ntrim, border_expand_rate, idx, raw_flag_range)
                })
                .collect(),
        )
    }

    fn flag_range(
        &self,
        ntrim: i32,
        border_expand_rate: f32,
        idx: usize,
        raw: &FlagRange,
    ) -> FlagRange {
        let true_values_sorted = self.true_values_sorted(idx);
        assert!(border_expand_rate < 0.4);
        assert!(border_expand_rate > 0.0);
        assert!(ntrim >= 0);
        let ntrim = ntrim as usize;
        let interval_width = raw.true_range.end - raw.true_range.start;
        let epsilon = interval_width * border_expand_rate;
        let start = if raw.ambiguous_start() {
            true_values_sorted[ntrim] - epsilon
        } else {
            raw.true_range.start
        };
        let end = if raw.ambiguous_end() {
            true_values_sorted[true_values_sorted.len() - 1 - ntrim] + epsilon
        } else {
            raw.true_range.end
        };
        FlagRange {
            true_range: ClosedRange { start, end },
            false_range: raw.false_range,
        }
    }

    fn true_values(&self, idx: usize) -> Vec<NotNan<f32>> {
        std::iter::zip(self.valuess[idx].iter(), self.flags.iter())
            .filter_map(|(value, flag)| if *flag { Some(*value) } else { None })
            .collect()
    }

    fn true_values_sorted(&self, idx: usize) -> Vec<NotNan<f32>> {
        let mut true_values = self.true_values(idx);
        true_values.sort();
        true_values
    }

    pub fn label0(&self) -> Label {
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
            within_true_range: self.true_range.contains(v),
            within_false_range: self.false_range.contains(v),
        }
    }

    pub fn from_values(
        values: impl Iterator<Item = NotNan<f32>> + Clone,
        flags: &[bool],
    ) -> Option<Self> {
        let values = values.into_iter();
        Some(Self {
            true_range: ClosedRange::from_values(
                std::iter::zip(values.clone(), flags.iter()).filter_map(|(value, &flag)| {
                    if flag {
                        Some(value)
                    } else {
                        None
                    }
                }),
            )?,
            false_range: ClosedRange::from_values(std::iter::zip(values, flags.iter()).filter_map(
                |(value, &flag)| {
                    if !flag {
                        Some(value)
                    } else {
                        None
                    }
                },
            ))
            .unwrap(),
        })
    }

    pub fn ambiguous_start(&self) -> bool {
        self.false_range.contains(self.true_range.start)
    }
    pub fn ambiguous_end(&self) -> bool {
        self.false_range.contains(self.true_range.end)
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    fn from_values(iter: impl Iterator<Item = T> + Clone) -> Option<Self> {
        let start = iter.clone().min()?;
        let end = iter.max().unwrap();
        Some(Self { start, end })
    }

    fn contains(&self, v: T) -> bool {
        self.start <= v && v <= self.end
    }
}
