use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum HollowTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<CustomTypePath, PreludeTypePath>,
        argument_tys: SmallVec<[FluffyTerm; 2]>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
    Hole(HoleSource, HoleKind),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType>,
        return_ty: FluffyTerm,
    },
}

impl HollowTerm {
    pub(crate) fn fluffy_data<'a>(self, hollow_terms: &'a HollowTerms) -> FluffyTermData<'a> {
        match hollow_terms.resolve_progress(self) {
            HollowTermResolveProgress::InProgress(term) => term.fluffy_data_aux(hollow_terms),
            HollowTermResolveProgress::Ok(_) => todo!(),
            HollowTermResolveProgress::Err(_) => todo!(),
        }
    }

    pub(crate) fn fluffy_data_aux<'a>(self, hollow_terms: &'a HollowTerms) -> FluffyTermData<'a> {
        match hollow_terms.data(self) {
            HollowTermData::TypeOntology {
                path,
                refined_path,
                argument_tys,
            } => FluffyTermData::TypeOntology {
                path: *path,
                refined_path: *refined_path,
                argument_tys,
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
            },
            HollowTermData::Hole(_, hole_kind) => FluffyTermData::Hole(*hole_kind, self),
            HollowTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}
