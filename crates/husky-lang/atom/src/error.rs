use std::sync::Arc;

use file::FileError;
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

impl From<FileError> for AtomError {
    fn from(_: FileError) -> Self {
        todo!()
    }
}

impl From<scope::ModuleFromFileError> for AtomError {
    fn from(_: scope::ModuleFromFileError) -> Self {
        todo!()
    }
}

pub type AtomResult<T> = Result<T, AtomError>;
pub type AtomResultArc<T> = Result<Arc<T>, AtomError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomRule {
    BeforeColonShouldBeScope,
    ListShouldBeWellFormed,
    ScopeShouldExist,
    AfterColonShouldBeUserDefinedIdentifier,
    NonEmptyAngles,
    ExpectCommaInAngle,
    AfterLAngleShouldBeCommaListOfScopes,
    KeywordShouldBeAtStart,
    CompatibleConvexity,
    ExpectTypeAfterLightArrow,
    BracketsShouldMatch,
    OnlyTypesInTheParenthesisBeforeLightArrow,
}
