use super::*;

/// should only use `Clone` in this crate
#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum SolidTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        // use fluffy term here because we don't want to recreate vectors when converting
        arguments: SmallVec<[FluffyTerm; 2]>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<SolidTerm>,
        parameter_ty: SolidTerm,
        return_ty: SolidTerm,
    },
    Ritchie {
        ritchie_kind: RitchieKind,
        // use fluffy term here because we don't want to recreate vectors when converting
        parameter_contracted_tys: SmallVec<[FluffyRitchieParameter; 2]>,
        return_ty: SolidTerm,
    },
}

impl<'a> From<&'a SolidTermData> for FluffyTermData<'a> {
    fn from(data: &'a SolidTermData) -> Self {
        match data {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FluffyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            SolidTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => FluffyTermData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_rune: todo!(),
                // parameter_rune.map(Into::into),
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            SolidTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}

impl<'a> Into<FluffyBaseTypeData<'a>> for &'a SolidTermData {
    fn into(self) -> FluffyBaseTypeData<'a> {
        match self {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FluffyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            SolidTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => FluffyBaseTypeData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_rune: todo!(),
                // parameter_rune.map(Into::into),
                parameter_ty: (*parameter_ty).into(),
                return_ty: (*return_ty).into(),
                ty_ethereal_term: None,
            },
            SolidTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}
