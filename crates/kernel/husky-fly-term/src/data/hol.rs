use husky_vfs::toolchain::Toolchain;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum HolTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, OtherTypePath>,
        arguments: SmallVec<[FlyTerm; 2]>,
    },
    Curry {
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
    },
    Hole {
        hole_source: HoleSource,
        hole_kind: HoleKind,
        fill: Option<FlyTerm>,
        constraints: SmallVec<[HoleConstraint; 2]>,
    },
    Ritchie {
        ritchie_kind: RitchieKind,
        params: Vec<FlyRitchieParameter>,
        return_ty: FlyTerm,
    },
}

/// refinement of HollowTerm
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hole(pub(crate) HolTerm);

impl Hole {
    pub fn term(self) -> HolTerm {
        self.0
    }
}

impl Hole {
    #[inline(always)]
    pub(crate) fn idx(self) -> usize {
        self.0.idx()
    }
}

impl Into<HolTerm> for Hole {
    fn into(self) -> HolTerm {
        self.0
    }
}

impl Into<FlyTerm> for Hole {
    fn into(self) -> FlyTerm {
        self.0.into()
    }
}

impl HolTerm {
    pub(crate) fn fly_data<'a>(
        self,
        db: &'a ::salsa::Db,
        fly_terms: &'a FlyTerms,
    ) -> FlyTermData<'a> {
        match self.resolve_progress(fly_terms) {
            TermResolveProgress::UnresolvedHol => self.fly_data_aux(db, fly_terms),
            TermResolveProgress::ResolvedEth(term) => ethereal_term_data(db, term),
            TermResolveProgress::ResolvedSol(term) => term.data2(fly_terms.sol_terms()).into(),
            TermResolveProgress::Err => todo!(),
        }
    }
    pub(crate) fn fly_data_aux<'a>(
        self,
        db: &'a ::salsa::Db,
        fly_terms: &'a FlyTerms,
    ) -> FlyTermData<'a> {
        match fly_terms.hol_terms().hollow_term_data(self) {
            HolTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            HolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => FlyTermData::Curry {
                toolchain: *toolchain,
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_hvar: *parameter_hvar,
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            HolTermData::Hole {
                fill: Some(fill), ..
            } => fill.base_term_data2(db, fly_terms),
            HolTermData::Hole {
                hole_kind,
                fill: None,
                ..
            } => FlyTermData::Hole(*hole_kind, Hole(self)),
            HolTermData::Ritchie {
                ritchie_kind,
                params: parameter_contracted_tys,
                return_ty,
            } => FlyTermData::Ritchie {
                ritchie_kind: *ritchie_kind,
                parameter_contracted_tys,
                return_ty: *return_ty,
            },
        }
    }

    pub(crate) fn fly_base_ty_data<'a>(
        self,
        db: &'a ::salsa::Db,
        fly_terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match self.resolve_progress(fly_terms) {
            TermResolveProgress::UnresolvedHol => self.fly_base_ty_data_aux(db, fly_terms),
            TermResolveProgress::ResolvedEth(term) => ethereal_term_fly_base_ty_data(db, term),
            TermResolveProgress::ResolvedSol(term) => term.data2(fly_terms.sol_terms()).into(),
            TermResolveProgress::Err => todo!(),
        }
    }

    pub(crate) fn fly_base_ty_data_aux<'a>(
        self,
        db: &'a ::salsa::Db,
        fly_terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match fly_terms.hol_terms().hollow_term_data(self) {
            HolTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            HolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => FlyBaseTypeData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_hvar: parameter_hvar.map(Into::into),
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            HolTermData::Hole {
                fill: Some(fill), ..
            } => fill.base_ty_data_inner(db, fly_terms),
            HolTermData::Hole {
                hole_kind,
                fill: None,
                ..
            } => FlyBaseTypeData::Hole(*hole_kind, Hole(self)),
            HolTermData::Ritchie {
                ritchie_kind,
                params: parameter_contracted_tys,
                return_ty,
            } => FlyBaseTypeData::Ritchie {
                ritchie_kind: *ritchie_kind,
                parameter_contracted_tys,
                return_ty: *return_ty,
            },
        }
    }
}
