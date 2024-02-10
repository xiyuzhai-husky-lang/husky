use super::*;

/// should only use `Clone` in this crate
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SolidTermData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        // use fly term here because we don't want to recreate vectors when converting
        arguments: SmallVec<[FlyTerm; 2]>,
    },
    Curry {
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_hvar: Option<SolTerm>,
        parameter_ty: SolTerm,
        return_ty: SolTerm,
    },
    Ritchie {
        ritchie_kind: RitchieKind,
        // use fly term here because we don't want to recreate vectors when converting
        parameter_contracted_tys: SmallVec<[FlyRitchieParameter; 2]>,
        return_ty: SolTerm,
    },
}

impl<'a> From<&'a SolidTermData> for FlyTermData<'a> {
    fn from(data: &'a SolidTermData) -> Self {
        match data {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            SolidTermData::Curry {
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
                parameter_hvar: todo!(),
                // parameter_hvar.map(Into::into),
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

impl<'a> Into<FlyBaseTypeData<'a>> for &'a SolidTermData {
    fn into(self) -> FlyBaseTypeData<'a> {
        match self {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments: argument_tys,
            } => FlyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: argument_tys,
                ty_ethereal_term: None,
            },
            SolidTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => FlyBaseTypeData::Curry {
                curry_kind: *curry_kind,
                variance: *variance,
                parameter_hvar: todo!(),
                // parameter_hvar.map(Into::into),
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
