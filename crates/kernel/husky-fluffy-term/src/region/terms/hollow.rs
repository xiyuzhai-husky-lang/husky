use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HollowTerms {
    pores: HoleRegistry,
    entries: Vec<HollowTermEntry>,
    first_unresolved_term: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HoleRegistry {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HoleIdn {}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTermEntry {
    src: HollowTermSource,
    data: HollowTermData,
    holes: VecSet<HoleIdn>,
    resolve_progress: FluffyTermResolveResult<Either<ResolvedTerm, HollowTerm>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HollowTerm {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HollowTermSource {
    Expectation(FluffyTermExpectationIdx),
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

// impl HollowTermEntry {
//     pub fn src(&self) -> ExprIdx {
//         self.src
//     }

//     pub fn unresolved_term(&self) -> &FluffyTermData {
//         &self.unresolved_term
//     }

//     pub fn original_error(&self) -> Option<&OriginalFluffyTermResolveError> {
//         match self.resolve_progress {
//             Err(FluffyTermResolveError::Original(ref e)) => Some(e),
//             _ => None,
//         }
//     }

//     pub(crate) fn resolve_progress(&self) -> Option<FluffyTerm> {
//         match self.resolve_progress {
//             Ok(resolve_progress) => Some(resolve_progress),
//             Err(_) => None,
//         }
//     }

//     pub(crate) fn is_done(&self) -> bool {
//         match self.resolve_progress {
//             Ok(FluffyTerm::Term(_) | FluffyTerm::PlaceType(_)) | Err(_) => true,
//             Ok(FluffyTerm::Unresolved(_)) => false,
//         }
//     }
// }
