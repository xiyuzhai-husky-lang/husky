use husky_entity_path::path::ItemPath;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum SemStaticVarDep {
    Item(ItemPath),
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemStaticVarDeps(OrderedSmallVecSet<SemStaticVarDep, 4>);

impl std::ops::Deref for SemStaticVarDeps {
    type Target = OrderedSmallVecSet<SemStaticVarDep, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &SemStaticVarDeps {
    type Item = SemStaticVarDep;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl SemStaticVarDeps {
    pub(crate) fn merge(&mut self, other: &[SemStaticVarDep]) {
        self.0.extend(other);
    }

    /// this is used when caching, to see whether there is any effective change
    pub(crate) fn merge_counted(
        &mut self,
        other: &[SemStaticVarDep],
        counter: &mut EffectiveMergeCounter,
    ) {
        let old_len = self.0.len();
        self.0.extend(other);
        if old_len != self.0.len() {
            counter.count += 1
        }
    }

    pub(crate) fn insert_item_path(&mut self, item_path: ItemPath) {
        self.0.insert(SemStaticVarDep::Item(item_path));
    }
}

/// None indicates no control flow
#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemControlFlowStaticVarDeps(Option<OrderedSmallVecSet<SemStaticVarDep, 4>>);

#[test]
fn sem_control_flow_static_var_deps_default_works() {
    assert_eq!(
        SemControlFlowStaticVarDeps::default(),
        SemControlFlowStaticVarDeps(None)
    );
}

impl std::ops::Deref for SemControlFlowStaticVarDeps {
    type Target = [SemStaticVarDep];

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Some(ref deps) => deps,
            None => &[],
        }
    }
}

impl SemControlFlowStaticVarDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => match other.0 {
                Some(ref other) => self.0 = Some(other.clone()),
                None => (),
            },
        }
    }

    /// use this when some control flow is caused by the value
    ///
    /// this will result in .0 always being Some
    pub(crate) fn merge_with_value(&mut self, other: &SemStaticVarDeps) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => self.0 = Some(other.0.clone()),
        }
    }

    pub(crate) fn merge_with_condition(&mut self, other: &SemStaticVarDeps) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => (), // doing nothing because if the thing doesn't have a control flow, so does its conditional version
        }
    }

    /// this is used when caching, to see whether there is any effective change
    pub(crate) fn merge_counted(&mut self, other: &Self, counter: &mut EffectiveMergeCounter) {
        match self.0 {
            Some(ref mut slf) => {
                let old_len = slf.len();
                slf.extend(other);
                if old_len != slf.len() {
                    counter.count += 1;
                }
            }
            None => match other.0 {
                Some(ref _other) => {
                    unreachable!("for the circumstance this function is going to be invoked, this case will not happen")
                    // self.0 = Some(other.clone());
                    // counter.count += 1;
                }
                None => (),
            },
        }
    }
}

#[derive(Default)]
pub(crate) struct EffectiveMergeCounter {
    count: usize,
}

impl EffectiveMergeCounter {
    pub fn count(&self) -> usize {
        self.count
    }
}
