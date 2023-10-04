pub trait RuleState {
    fn is_resolved(&self) -> bool;
}

pub struct RuleTable<R, S: RuleState> {
    entries: Vec<(R, S)>,
    last_unresolved: usize,
}

impl<R, S: RuleState> RuleTable<R, S> {}
