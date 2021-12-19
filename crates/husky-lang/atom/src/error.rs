use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AtomError {
    pub range: TextRange,
    pub rule_broken: AtomRule,
}

impl AtomError {
    pub fn new(range: TextRange, rule_broken: AtomRule) -> AtomError {
        AtomError { range, rule_broken }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomRule {
    BeforeColonShouldBeScope,
    ScopeShouldExist,
    AfterColonShouldBeIdentifier,
    GenericArgumentsShouldBeNonEmpty,
    AfterLAngleShouldBeCommaListOfScopes,
    KeywordShouldBeAtStart,
    CompatibleConvexity,
}
