mod hollow;
mod solid;

pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;

pub enum FluffyTermData {
    Literal(TermLiteral),
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
    Hole(HoleKind, HollowTerm),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameterContractedType>,
        return_ty: FluffyTerm,
    },
}

impl FluffyTerm {
    pub fn data<'a>(self, db: &'a impl FluffyTermEngine) -> &'a FluffyTermData {
        todo!()
    }

    pub(crate) fn data_inner<'a>(
        self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> &'a FluffyTermData {
        todo!()
    }
}
