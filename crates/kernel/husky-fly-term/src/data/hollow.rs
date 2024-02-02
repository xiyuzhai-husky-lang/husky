use husky_vfs::Toolchain;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum HollowTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FlyTerm; 2]>,
    },
    Curry {
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
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
pub struct Hole(pub(crate) HollowTerm);

impl Hole {
    pub fn term(self) -> HollowTerm {
        self.0
    }
}

impl Hole {
    #[inline(always)]
    pub(crate) fn idx(self) -> usize {
        self.0.idx()
    }
}

impl Into<HollowTerm> for Hole {
    fn into(self) -> HollowTerm {
        self.0
    }
}

impl Into<FlyTerm> for Hole {
    fn into(self) -> FlyTerm {
        self.0.into()
    }
}

impl HollowTerm {
    pub(crate) fn fluffy_data<'a>(
        self,
        db: &'a ::salsa::Db,
        fluffy_terms: &'a FlyTerms,
    ) -> FlyTermData<'a> {
        match self.resolve_progress(fluffy_terms) {
            TermResolveProgress::UnresolvedHollow => self.fluffy_data_aux(db, fluffy_terms),
            TermResolveProgress::ResolvedEthereal(term) => ethereal_term_data(db, term),
            TermResolveProgress::ResolvedSolid(term) => {
                term.data_inner(fluffy_terms.solid_terms()).into()
            }
            TermResolveProgress::Err => todo!(),
        }
    }
    pub(crate) fn fluffy_data_aux<'a>(
        self,
        db: &'a ::salsa::Db,
        fluffy_terms: &'a FlyTerms,
    ) -> FlyTermData<'a> {
        match fluffy_terms.hollow_terms().hollow_term_data(self) {
            HollowTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            HollowTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => FlyTermData::Curry {
                toolchain: *toolchain,
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_rune: *parameter_rune,
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            HollowTermData::Hole {
                fill: Some(fill), ..
            } => fill.data_inner(db, fluffy_terms),
            HollowTermData::Hole {
                hole_kind,
                fill: None,
                ..
            } => FlyTermData::Hole(*hole_kind, Hole(self)),
            HollowTermData::Ritchie {
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

    pub(crate) fn fluffy_base_ty_data<'a>(
        self,
        db: &'a ::salsa::Db,
        fluffy_terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match self.resolve_progress(fluffy_terms) {
            TermResolveProgress::UnresolvedHollow => self.fluffy_base_ty_data_aux(db, fluffy_terms),
            TermResolveProgress::ResolvedEthereal(term) => {
                ethereal_term_fluffy_base_ty_data(db, term)
            }
            TermResolveProgress::ResolvedSolid(term) => {
                term.data_inner(fluffy_terms.solid_terms()).into()
            }
            TermResolveProgress::Err => todo!(),
        }
    }

    pub(crate) fn fluffy_base_ty_data_aux<'a>(
        self,
        db: &'a ::salsa::Db,
        fluffy_terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match fluffy_terms.hollow_terms().hollow_term_data(self) {
            HollowTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            HollowTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => FlyBaseTypeData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_rune: parameter_rune.map(Into::into),
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            HollowTermData::Hole {
                fill: Some(fill), ..
            } => fill.base_ty_data_inner(db, fluffy_terms),
            HollowTermData::Hole {
                hole_kind,
                fill: None,
                ..
            } => FlyBaseTypeData::Hole(*hole_kind, Hole(self)),
            HollowTermData::Ritchie {
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
