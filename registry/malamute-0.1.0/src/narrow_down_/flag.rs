use super::*;
use ad_hoc_task_dependency::IsLabel;
use ad_hoc_task_dependency::{ugly::__InputId, val_control_flow::ValControlFlow};
use smallvec::*;

pub struct FlagVectorField<Label> {
    // flags: Vec<bool>,
    stalks: Vec<Stalk>,
    label0: Label,
}

pub struct Stalk {
    features: SmallVec<[NotNan<f32>; 4]>,
    // indicates whether the label is equal to label0
    flag: bool,
}

impl<Label> FlagVectorField<Label>
where
    Label: IsLabel,
{
    pub fn from_features(
        val_domain_repr: __ValDomainReprInterface,
        arguments: &[__ValReprInterface],
        label0: Label,
    ) -> Result<Self, ()> {
        let mut stalks: Vec<Stalk> = vec![];
        for i in 0..500 {
            let input_id = __InputId::from_index(i);
            if let Some(stalk) =
                Self::from_features_aux(val_domain_repr, input_id, arguments, label0)?
            {
                stalks.push(stalk)
            }
        }
        Ok(Self { stalks, label0 })
    }

    fn from_features_aux(
        val_domain_repr: __ValDomainReprInterface,
        input_id: __InputId,
        arguments: &[__ValReprInterface],
        label0: Label,
    ) -> Result<Option<Stalk>, ()> {
        match __eval_val_domain_repr_at_input(val_domain_repr, input_id, None) {
            ValControlFlow::Continue(_) => (),
            ValControlFlow::LoopContinue => todo!(),
            ValControlFlow::LoopBreak(_) => todo!(),
            ValControlFlow::Return(_) => todo!(),
            ValControlFlow::Undefined => todo!(),
            ValControlFlow::Err(_) => todo!(),
        };
        let mut features: SmallVec<[NotNan<f32>; 4]> = smallvec![];
        for &argument in arguments {
            let feature = match __eval_val_repr_at_input(argument, input_id, None) {
                ValControlFlow::Continue(feature) => feature,
                ValControlFlow::LoopContinue => todo!(),
                ValControlFlow::LoopBreak(_) => todo!(),
                ValControlFlow::Return(_) => todo!(),
                ValControlFlow::Undefined => todo!(),
                ValControlFlow::Err(_) => todo!(),
            };
            let feature = match NotNan::new(feature) {
                Ok(feature) => feature,
                Err(_) => todo!(),
            };
            features.push(feature)
        }
        Ok(Some(Stalk {
            features,
            flag: Label::label_at_input(input_id) == label0,
        }))
    }

    fn raw_flag_ranges(&self) -> SmallVec<[FlagRange; 4]> {
        if self.stalks.is_empty() {
            return smallvec![];
        }
        let num_of_features = self.stalks[0].features.len();
        (0..num_of_features)
            .into_iter()
            .filter_map(|i| {
                FlagRange::from_value_flag_pairs(
                    self.stalks
                        .iter()
                        .map(|stalk| (stalk.features[i], stalk.flag)),
                )
            })
            .collect()
    }

    pub fn flag_ranges(&self, ntrim: i32, border_expand_rate: f32) -> SmallVec<[FlagRange; 4]> {
        let raw_flag_ranges = self.raw_flag_ranges();
        raw_flag_ranges
            .iter()
            .enumerate()
            .map(|(idx, raw_flag_range)| {
                self.flag_range(ntrim, border_expand_rate, idx, raw_flag_range)
            })
            .collect()
    }

    fn flag_range(
        &self,
        skip: i32,
        border_expand_rate: f32,
        idx: usize,
        raw: &FlagRange,
    ) -> FlagRange {
        let true_values_sorted = self.true_values_sorted(idx);
        assert!(border_expand_rate < 0.4);
        assert!(border_expand_rate > 0.0);
        assert!(skip >= 0);
        let skip = skip as usize;
        let interval_width = raw.true_range.end - raw.true_range.start;
        let epsilon = interval_width * border_expand_rate;
        let start = if raw.ambiguous_start() {
            true_values_sorted[skip] - epsilon
        } else {
            raw.true_range.start
        };
        let end = if raw.ambiguous_end() {
            true_values_sorted[true_values_sorted.len() - 1 - skip] + epsilon
        } else {
            raw.true_range.end
        };
        FlagRange {
            true_range: ClosedRange { start, end },
            false_range: raw.false_range,
        }
    }

    fn true_values(&self, idx: usize) -> Vec<NotNan<f32>> {
        self.stalks
            .iter()
            .map(|stalk| (stalk.features[idx], stalk.flag))
            .filter_map(|(value, flag)| if flag { Some(value) } else { None })
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

    pub fn from_value_flag_pairs(
        value_flag_pairs: impl Iterator<Item = (NotNan<f32>, bool)> + Clone,
    ) -> Option<Self> {
        Some(Self {
            true_range: ClosedRange::from_values(value_flag_pairs.clone().filter_map(
                |(value, flag)| {
                    if flag {
                        Some(value)
                    } else {
                        None
                    }
                },
            ))?,
            false_range: ClosedRange::from_values(value_flag_pairs.filter_map(|(value, flag)| {
                if !flag {
                    Some(value)
                } else {
                    None
                }
            }))
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
