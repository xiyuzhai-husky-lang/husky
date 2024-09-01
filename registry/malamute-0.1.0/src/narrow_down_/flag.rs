use super::*;
use ad_hoc_devsoul_dependency::IsLabel;
use ml_task::IsMlTask;
use smallvec::*;

#[derive(Debug)]
pub struct FlagVectorField<Label> {
    number_of_features: usize,
    stalks: Vec<Stalk>,
    label0: Label,
}

#[derive(Debug)]
pub struct Stalk {
    features: SmallVec<[NotNan<f32>; 4]>,
    // indicates whether the label is equal to label0
    flag: bool,
}

impl<Label> FlagVectorField<Label>
where
    Label: IsLabel,
{
    pub fn from_features<Task: IsMlTask<__VarId>>(
        ki_domain_repr: __KiDomainReprInterface,
        features: &[__KiReprInterface],
        label0: Label,
    ) -> Result<Self, __TrackedException> {
        let mut stalks: Vec<Stalk> = vec![];
        let locked = &[]; // double check that this is correct
        let mut ids = Task::INPUT::page_var_ids(locked, 0u32.into(), None);
        for i in 0..5 {
            let Some(id) = ids.next() else { break };
            Task::INPUT::with_var_id(id, locked, || {
                if let Some(stalk) = Self::from_features_aux(ki_domain_repr, features, label0)? {
                    stalks.push(stalk)
                }
                Ok(())
            })
            .unwrap()?
        }
        Ok(Self {
            number_of_features: features.len(),
            stalks,
            label0,
        })
    }

    fn from_features_aux(
        ki_domain_repr: __KiDomainReprInterface,
        arguments: &[__KiReprInterface],
        label0: Label,
    ) -> Result<Option<Stalk>, __TrackedException> {
        match __eval_ki_domain_repr_interface(ki_domain_repr) {
            __KiControlFlow::Continue(_) => (),
            __KiControlFlow::LoopContinue => todo!(),
            __KiControlFlow::LoopExit(_) => todo!(),
            __KiControlFlow::Return(_) => todo!(),
            __KiControlFlow::Undefined => return Ok(None),
            __KiControlFlow::Throw(e) => return Err(e),
        };
        let mut features: SmallVec<[NotNan<f32>; 4]> = smallvec![];
        for &argument in arguments {
            let feature = match __eval_ki_repr_interface(argument, None) {
                __KiControlFlow::Continue(feature) => feature,
                __KiControlFlow::LoopContinue => todo!(),
                __KiControlFlow::LoopExit(_) => todo!(),
                __KiControlFlow::Return(_) => todo!(),
                __KiControlFlow::Undefined => todo!(),
                __KiControlFlow::Throw(e) => return Err(e),
            };
            let feature = match NotNan::new(feature) {
                Ok(feature) => feature,
                Err(_) => todo!(),
            };
            features.push(feature)
        }
        Ok(Some(Stalk {
            features,
            flag: Label::label() == label0,
        }))
    }

    fn raw_flag_ranges(&self) -> SmallVec<[Option<FlagRange>; 4]> {
        if self.stalks.is_empty() {
            return (0..self.number_of_features).map(|_| None).collect();
            // return smallvec![];
        }
        let num_of_features = self.stalks[0].features.len();
        (0..num_of_features)
            .into_iter()
            .map(|i| {
                FlagRange::from_value_flag_pairs(
                    self.stalks
                        .iter()
                        .map(|stalk| (stalk.features[i], stalk.flag)),
                )
            })
            .collect()
    }

    pub fn flag_ranges(
        &self,
        ntrim: i32,
        border_expand_rate: f32,
    ) -> SmallVec<[Option<FlagRange>; 4]> {
        let raw_flag_ranges = self.raw_flag_ranges();
        raw_flag_ranges
            .iter()
            .enumerate()
            .map(|(idx, raw_flag_range)| {
                self.flag_range(ntrim, border_expand_rate, idx, raw_flag_range.as_ref()?)
            })
            .collect()
    }

    fn flag_range(
        &self,
        skip: i32,
        border_expand_rate: f32,
        idx: usize,
        raw: &FlagRange,
    ) -> Option<FlagRange> {
        let true_values_sorted = self.true_values_sorted(idx);
        assert!(border_expand_rate < 0.4);
        assert!(border_expand_rate > 0.0);
        assert!(skip >= 0);
        let skip = skip as usize;
        if skip >= true_values_sorted.len() {
            return None;
        }
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
        Some(FlagRange {
            true_range: ClosedRange { start, end },
            false_range: raw.false_range,
        })
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

#[derive(Debug, PartialEq, Eq, Clone, __Serialize)]
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
            }))?,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, __Serialize)]
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
