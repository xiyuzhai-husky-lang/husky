use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerms {
    entries: Vec<HollowTermEntry>,
    first_unresolved_term: usize,
}

impl HollowTerms {
    pub fn entries(&self) -> &[HollowTermEntry] {
        self.entries.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTermEntry {
    src: HollowTermSource,
    data: HollowTermData,
    resolve_progress: FluffyTermResolveResult<Either<ResolvedTerm, HollowTerm>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerm {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum HollowTermSource {
    Expr(ExprIdx),
    Expectation(FluffyTermExpectationIdx),
}

impl HollowTermSource {
    pub fn expr_idx(self) -> ExprIdx {
        todo!()
        // self.expr_idx
    }
}
use super::*;
use vec_like::VecSet;

impl HollowTermEntry {
    pub(super) fn force_resolve_term(&mut self) -> Option<ResolvedTerm> {
        todo!()
        // match self.resolve_progress {
        //     Ok(FluffyTerm::Term(term)) => Some(term),
        //     Ok(FluffyTerm::Unresolved(_)) => {
        //         self.resolve_progress = Err(OriginalFluffyTermResolveError::UnresolvedTerm.into());
        //         None
        //     }
        //     Ok(FluffyTerm::PlaceType(_)) => todo!(),
        //     Err(_) => None,
        // }
    }
}

impl HollowTermEntry {
    pub fn src(&self) -> HollowTermSource {
        self.src
    }

    pub fn data(&self) -> &HollowTermData {
        &self.data
    }

    pub fn original_error(&self) -> Option<&OriginalFluffyTermResolveError> {
        match self.resolve_progress {
            Err(FluffyTermResolveError::Original(ref e)) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn resolve_progress(&self) -> Option<FluffyTerm> {
        todo!()
        // match self.resolve_progress {
        //     Ok(resolve_progress) => Some(resolve_progress),
        //     Err(_) => None,
        // }
    }

    pub(crate) fn is_done(&self) -> bool {
        todo!()
        // match self.resolve_progress {
        //     Ok(FluffyTerm::Term(_) | FluffyTerm::PlaceType(_)) | Err(_) => true,
        //     Ok(FluffyTerm::Unresolved(_)) => false,
        // }
    }
}
