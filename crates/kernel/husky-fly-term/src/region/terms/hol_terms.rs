use husky_eth_term::term::{
    curry::EthCurry,
    ritchie::{
        EthRitchie, EthRitchieSimpleParameter, EtherealRitchieKeyedParameter,
        EtherealRitchieVariadicParameter,
    },
};

use super::*;

// `Default` is derived because we never inherited hollow terms
#[salsa::debug_with_db]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HolTerms {
    entries: Vec<HolTermEntry>,
    first_unresolved_term_idx: usize,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum HoleConstraint {
    CoercibleFrom { target: FlyTerm },
    CoercibleInto { target: FlyTerm },
    Subtype { target: FlyTerm },
    Supertype { target: FlyTerm },
}

impl HolTerms {
    // for ide
    pub fn errors(&self) -> impl Iterator<Item = (HoleSource, &OriginalHollowTermResolveError)> {
        self.entries.iter().filter_map(|entry| match entry {
            HolTermEntry {
                data: HolTermData::Hole { hole_source, .. },
                resolve_progress:
                    HolTermResolveProgressBuf::Err(HollowTermResolveError::Original(e)),
            } => Some((*hole_source, e)),
            _ => None,
        })
    }

    // alloc something that's actually different
    #[inline(always)]
    pub(crate) fn alloc_new(&mut self, data: HolTermData) -> HolTerm {
        let idx = self.entries.len();
        self.entries.push(HolTermEntry {
            data,
            resolve_progress: HolTermResolveProgressBuf::Unresolved,
        });
        HolTerm(idx.try_into().expect("within range"))
    }

    pub(crate) fn hollow_term_data(&self, hollow_term: HolTerm) -> &HolTermData {
        &self.entry(hollow_term).data
    }

    pub(crate) fn entry(&self, hollow_term: HolTerm) -> &HolTermEntry {
        &self.entries[hollow_term.idx()]
    }

    pub(crate) fn add_hole_constraint(&mut self, hole: Hole, hole_constraint: HoleConstraint) {
        let mut hole_entry = &mut self.entries[hole.idx()];
        match hole_entry.data {
            HolTermData::Hole {
                ref mut constraints,
                ..
            } => constraints.push(hole_constraint),
            _ => unreachable!(),
        }
    }

    pub(crate) fn empty_holes_with_non_empty_constraints<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Hole, HoleKind, &'a [HoleConstraint])> + 'a {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| match entry.data {
                HolTermData::Hole {
                    hole_source,
                    hole_kind,
                    fill: None,
                    ref constraints,
                } => (constraints.len() > 0).then_some((
                    Hole(HolTerm(i as u32)),
                    hole_kind,
                    constraints as &[_],
                )),
                _ => None,
            })
    }

    fn update_entries(&mut self, db: &::salsa::Db, solid_terms: &mut SolTerms) {
        let first_unresolved_idx = self.get_first_unresolved_term_idx();
        for idx in first_unresolved_idx..self.entries.len() {
            self.try_update_entry(db, solid_terms, idx)
        }
    }

    // lazy update
    fn get_first_unresolved_term_idx(&mut self) -> usize {
        let mut idx = self.first_unresolved_term_idx;
        // lazy update
        while idx < self.entries.len() {
            match self.entries[idx].is_resolved() {
                true => idx += 1,
                false => break,
            }
        }
        self.first_unresolved_term_idx = idx;
        idx
    }

    fn try_update_entry(&mut self, db: &::salsa::Db, solid_terms: &mut SolTerms, idx: usize) {
        if self.entries[idx].is_resolved() {
            return;
        }
        match self.entries[idx].resolve_progress {
            HolTermResolveProgressBuf::ResolvedEthereal(_)
            | HolTermResolveProgressBuf::ResolvedSolid(_) => return,
            _ => (),
        }
        let mut merger = FlyTermDataKindMerger::new(self);
        match self.entries[idx].data {
            HolTermData::TypeOntology {
                path,
                refined_path,
                ref arguments,
            } => {
                // todo: use merger
                let mut solid_flag = false;
                for argument in arguments {
                    match argument.resolve_progress(self) {
                        // we can't proceed if any argument is unresolved hollow
                        TermResolveProgress::UnresolvedHol => return,
                        TermResolveProgress::ResolvedEth(_) => (),
                        TermResolveProgress::ResolvedSol(_) => solid_flag = true,
                        TermResolveProgress::Err => todo!(),
                    }
                }
                if solid_flag {
                    p!(path.debug(db));
                    todo!()
                } else {
                    self.entries[idx].resolve_progress = match EthTerm::new_ty_ontology(
                        db,
                        path,
                        arguments
                            .iter()
                            .map(|argument| match argument.resolve_progress(self) {
                                TermResolveProgress::ResolvedEth(argument) => argument,
                                _ => unreachable!(),
                            }),
                    ) {
                        Ok(term) => HolTermResolveProgressBuf::ResolvedEthereal(term),
                        Err(_) => todo!(),
                    }
                }
            }
            HolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => {
                merger.accept(parameter_hvar.map(|hvar| *hvar));
                merger.accept_one(parameter_ty);
                merger.accept_one(return_ty);
                match merger.data_kind() {
                    FlyTermDataKind::Err => todo!(),
                    FlyTermDataKind::Ethereal => {
                        let parameter_hvar = parameter_hvar.map(|parameter_hvar| {
                            parameter_hvar.resolve_as_ethereal(self).unwrap().hvar()
                        });
                        let parameter_ty = parameter_ty.resolve_as_ethereal(self).unwrap();
                        let return_ty = return_ty.resolve_as_ethereal(self).unwrap();
                        self.entries[idx].resolve_progress =
                            HolTermResolveProgressBuf::ResolvedEthereal(
                                EthCurry::new(
                                    toolchain,
                                    curry_kind,
                                    variance,
                                    parameter_hvar,
                                    parameter_ty,
                                    return_ty,
                                    db,
                                )
                                .into(),
                            )
                    }
                    FlyTermDataKind::Solid => todo!(),
                    FlyTermDataKind::Hollow => return,
                }
            }
            HolTermData::Hole { fill, .. } => match fill {
                Some(fill) => match fill.resolve_progress(self) {
                    TermResolveProgress::UnresolvedHol => (),
                    TermResolveProgress::ResolvedEth(term) => {
                        self.entries[idx].resolve_progress =
                            HolTermResolveProgressBuf::ResolvedEthereal(term)
                    }
                    TermResolveProgress::ResolvedSol(_) => todo!(),
                    TermResolveProgress::Err => todo!(),
                },
                None => (),
            },
            HolTermData::Ritchie {
                ritchie_kind,
                ref params,
                return_ty,
            } => {
                let mut solid_flag = false;
                for param in params {
                    match param.ty().resolve_progress(self) {
                        // we can't proceed if any argument is unresolved hollow
                        TermResolveProgress::UnresolvedHol => return,
                        TermResolveProgress::ResolvedEth(_) => (),
                        TermResolveProgress::ResolvedSol(_) => solid_flag = true,
                        TermResolveProgress::Err => todo!(),
                    }
                }
                match return_ty.resolve_progress(self) {
                    TermResolveProgress::UnresolvedHol => return,
                    TermResolveProgress::ResolvedEth(_) => (),
                    TermResolveProgress::ResolvedSol(_) => todo!(),
                    TermResolveProgress::Err => todo!(),
                }
                if solid_flag {
                    todo!()
                } else {
                    let params = params.iter().map(|param| {
                        let TermResolveProgress::ResolvedEth(ty) =
                            param.ty().resolve_progress(self)
                        else {
                            unreachable!()
                        };
                        match param {
                            FlyRitchieParameter::Regular(param) => {
                                EthRitchieSimpleParameter::new(param.contract(), ty).into()
                            }
                            FlyRitchieParameter::Variadic(param) => {
                                EtherealRitchieVariadicParameter::new(param.contract(), ty).into()
                            }
                            FlyRitchieParameter::Keyed(param) => {
                                EtherealRitchieKeyedParameter::new(
                                    param.key(),
                                    param.contract(),
                                    ty,
                                    param.has_default(),
                                )
                                .into()
                            }
                        }
                    });
                    let return_ty = match return_ty.resolve_progress(self) {
                        TermResolveProgress::ResolvedEth(return_ty) => return_ty,
                        _ => unreachable!(),
                    };
                    self.entries[idx].resolve_progress =
                        match EthRitchie::new(db, ritchie_kind, params, return_ty) {
                            Ok(term) => HolTermResolveProgressBuf::ResolvedEthereal(term.into()),
                            Err(_) => todo!(),
                        }
                }
            }
        }
    }
}

impl FlyTerms {
    pub(crate) fn fill_hole(&mut self, db: &::salsa::Db, hole: Hole, term: FlyTerm) {
        self.fill_hole_aux(hole.idx(), term, db)
    }

    fn fill_hole_aux(&mut self, hole_idx: usize, term: FlyTerm, db: &::salsa::Db) {
        let mut hole_entry = &mut self.hollow_terms.entries[hole_idx];
        match hole_entry.data {
            HolTermData::Hole { fill: Some(_), .. } => unreachable!(),
            HolTermData::Hole { ref mut fill, .. } => *fill = Some(term),
            _ => unreachable!(),
        }
        // update progress if term is resolved
        match term.base() {
            FlyTermBase::Eth(term) => {
                hole_entry.resolve_progress = HolTermResolveProgressBuf::ResolvedEthereal(term)
            }
            FlyTermBase::Sol(term) => {
                hole_entry.resolve_progress = HolTermResolveProgressBuf::ResolvedSolid(term)
            }
            FlyTermBase::Hol(_) => (),
            FlyTermBase::Place => todo!(),
        }
        self.hollow_terms.update_entries(db, &mut self.solid_terms)
    }

    pub(crate) fn fill_hole_by_force(
        &mut self,
        hole: Hole,
        db: &::salsa::Db,
        term_menu: &EthTermMenu,
    ) {
        let mut hole_entry = &mut self.hollow_terms.entries[hole.idx()];
        let HolTermData::Hole {
            hole_kind,
            ref constraints,
            ..
        } = hole_entry.data
        else {
            unreachable!()
        };
        // todo: for constraint in constraints
        let term = match hole_kind {
            HoleKind::UnspecifiedIntegerType => term_menu.i32_ty_ontology().into(),
            HoleKind::UnspecifiedFloatType => term_menu.f32_ty_ontology().into(),
            HoleKind::ImplicitType => return, // ad hoc
            HoleKind::AnyOriginal => return,  // ad hoc
            HoleKind::AnyDerived => return,   // ad hoc
        };
        self.fill_hole(db, hole, term)
    }

    pub(in crate::region) fn fill_all_holes(&mut self, db: &::salsa::Db, term_menu: &EthTermMenu) {
        // we know that no new holes are generated
        for idx in 0..self.hollow_terms.entries.len() {
            match self.hollow_terms.entries[idx].data {
                HolTermData::Hole {
                    hole_source,
                    hole_kind,
                    fill: None,
                    ref constraints,
                } => self.fill_hole_by_force(Hole(HolTerm(idx as u32)), db, term_menu),
                _ => continue,
            }
        }
    }
}

impl HolTerm {
    pub(crate) fn resolve_progress(
        self,
        hollow_terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> TermResolveProgress {
        hollow_terms.borrow().entries[self.idx()]
            .resolve_progress
            .share()
    }
}

impl FlyTerm {
    pub(crate) fn resolve_progress(
        self,
        terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> TermResolveProgress {
        match self.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => TermResolveProgress::ResolvedEth(term),
            FlyTermBase::Sol(term) => TermResolveProgress::ResolvedSol(term),
            FlyTermBase::Hol(term) => term.resolve_progress(terms.borrow()),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> Option<EthTerm> {
        match self.resolve_progress(terms) {
            TermResolveProgress::ResolvedEth(term) => Some(term),
            _ => None,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct HolTermEntry {
    data: HolTermData,
    resolve_progress: HolTermResolveProgressBuf,
}

impl HolTermEntry {
    pub fn data(&self) -> &HolTermData {
        &self.data
    }

    pub(crate) fn is_resolved(&self) -> bool {
        match self.resolve_progress {
            HolTermResolveProgressBuf::Unresolved => false,
            _ => true,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TermResolveProgress {
    UnresolvedHol,
    ResolvedEth(EthTerm),
    ResolvedSol(SolTerm),
    Err,
}

impl HolTermResolveProgressBuf {
    fn share(&self) -> TermResolveProgress {
        match self {
            HolTermResolveProgressBuf::Unresolved => TermResolveProgress::UnresolvedHol,
            HolTermResolveProgressBuf::ResolvedEthereal(term) => {
                TermResolveProgress::ResolvedEth(*term)
            }
            HolTermResolveProgressBuf::ResolvedSolid(term) => {
                TermResolveProgress::ResolvedSol(*term)
            }
            HolTermResolveProgressBuf::Err(_) => TermResolveProgress::Err,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum HolTermResolveProgressBuf {
    Unresolved,
    ResolvedEthereal(EthTerm),
    ResolvedSolid(SolTerm),
    Err(HollowTermResolveError),
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HolTerm(u32);

impl HolTerm {
    #[inline(always)]
    pub(crate) fn idx(self) -> usize {
        self.0 as usize
    }
}
