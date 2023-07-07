use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum HollowTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FluffyTerm; 2]>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
    Hole {
        hole_source: HoleSource,
        hole_kind: HoleKind,
        fill: Option<FluffyTerm>,
        constraints: SmallVec<[HoleConstraint; 2]>,
    },
    Ritchie {
        ritchie_kind: RitchieKind,
        params: Vec<FluffyTermRitchieParameter>,
        return_ty: FluffyTerm,
    },
    PlaceTypeOntology {
        place: Place,
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FluffyTerm; 2]>,
    },
    PlaceHole {
        place: Place,
        hole_kind: HoleKind,
        hole: Hole,
    },
}

/// refinement of HollowTerm
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hole(HollowTerm);

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

impl Into<FluffyTerm> for Hole {
    fn into(self) -> FluffyTerm {
        self.0.into()
    }
}

impl HollowTerm {
    pub(crate) fn fluffy_data<'a>(
        self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> FluffyTermData<'a> {
        match self.resolve_progress(fluffy_terms) {
            TermResolveProgress::UnresolvedHollow => self.fluffy_data_aux(db, fluffy_terms),
            TermResolveProgress::ResolvedEthereal(term) => ethereal_term_data(db, term),
            TermResolveProgress::ResolvedSolid(_) => todo!(),
            TermResolveProgress::Err => todo!(),
        }
    }

    pub(crate) fn fluffy_data_aux<'a>(
        self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> FluffyTermData<'a> {
        match fluffy_terms.hollow_terms().data(self) {
            HollowTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FluffyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                arguments: argument_tys,
                ty_ethereal_term: None,
            },
            HollowTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => FluffyTermData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_variable: parameter_variable.map(Into::into),
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
            } => FluffyTermData::Hole(*hole_kind, Hole(self)),
            HollowTermData::Ritchie {
                ritchie_kind,
                params: parameter_contracted_tys,
                return_ty,
            } => FluffyTermData::Ritchie {
                ritchie_kind: *ritchie_kind,
                parameter_contracted_tys,
                return_ty: *return_ty,
            },
            HollowTermData::PlaceTypeOntology {
                place,
                path,
                refined_path,
                arguments,
            } => todo!(),
            HollowTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => FluffyTermData::HoleAtPlace {
                place: *place,
                hole_kind: *hole_kind,
                hole: *hole,
            },
        }
    }
}
