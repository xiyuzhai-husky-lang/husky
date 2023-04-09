use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSource {
    Expr(ExprIdx),
    ExpectationResolve { parent: FluffyTermExpectationIdx },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyTermExpectationEntry {
    src: HollowTermSource,
    expectee: FluffyTerm,
    data: FluffyTermExpectationData,
    resolve_progress: FluffyTermExpectationResolveProgress,
}

impl FluffyTermExpectationEntry {
    pub fn src(&self) -> HollowTermSource {
        self.src
    }

    pub(crate) fn expectee(&self) -> FluffyTerm {
        self.expectee
    }

    pub(crate) fn data(&self) -> &FluffyTermExpectationData {
        &self.data
    }

    pub fn resolve_progress(&self) -> &FluffyTermExpectationResolveProgress {
        &self.resolve_progress
    }

    pub fn original_error(&self) -> Option<&OriginalFluffyTermExpectationError> {
        match self.resolve_progress {
            FluffyTermExpectationResolveProgress::Resolved(Err(
                FluffyTermExpectationError::Original(ref e),
            )) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: FluffyTermExpectationResult<FluffyTermExpectationOutcome>,
    ) {
        self.resolve_progress = FluffyTermExpectationResolveProgress::Resolved(result)
    }
}
