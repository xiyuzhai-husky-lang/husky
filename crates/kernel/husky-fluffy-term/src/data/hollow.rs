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
    Hole(HoleKind),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType>,
        return_ty: FluffyTerm,
    },
}
