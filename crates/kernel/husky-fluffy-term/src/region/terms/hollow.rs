use super::*;

// `Default` is derived because we never inherited hollow terms
#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerms {
    entries: Vec<HollowTermEntry>,
    first_unresolved_term: usize,
}

impl HollowTerms {
    // for ide
    pub fn errors(&self) -> impl Iterator<Item = (HoleSource, &OriginalHollowTermResolveError)> {
        self.entries.iter().filter_map(|entry| match entry {
            HollowTermEntry {
                data: HollowTermData::Hole(src, _),
                resolve_progress:
                    HollowTermResolveProgress::Err(HollowTermResolveError::Original(e)),
            } => Some((*src, e)),
            _ => None,
        })
    }

    // alloc something that's actually different
    #[inline(always)]
    pub(crate) fn alloc_new(&mut self, data: HollowTermData) -> HollowTerm {
        let idx = self.entries.len();
        let term = HollowTerm(idx.try_into().expect("within range"));
        self.entries.push(HollowTermEntry {
            data,
            resolve_progress: HollowTermResolveProgress::InProgress(term),
        });
        term
    }

    pub(crate) fn resolve_progress(&self, hollow_term: HollowTerm) -> &HollowTermResolveProgress {
        &self.entry(hollow_term).resolve_progress
    }

    pub(crate) fn data(&self, hollow_term: HollowTerm) -> &HollowTermData {
        &self.entry(hollow_term).data
    }

    fn entry(&self, hollow_term: HollowTerm) -> &HollowTermEntry {
        &self.entries[hollow_term.0 as usize]
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTermEntry {
    data: HollowTermData,
    resolve_progress: HollowTermResolveProgress,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum HollowTermResolveProgress {
    InProgress(HollowTerm),
    Ok(NestedFluffyTerm),
    Err(HollowTermResolveError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct HollowTerm(u32);

use super::*;
use vec_like::VecSet;

impl HollowTermEntry {
    pub(super) fn force_resolve_term(&mut self) -> Option<NestedFluffyTerm> {
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
    pub fn data(&self) -> &HollowTermData {
        &self.data
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
