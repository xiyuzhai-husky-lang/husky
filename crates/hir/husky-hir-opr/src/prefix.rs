use either::*;
use husky_entity_path::{PreludeNumTypePath, PreludeTypePath};
use husky_fluffy_term::FluffyTerm;
use husky_fluffy_term::FluffyTerms;
use husky_sema_opr::prefix::SemaPrefixOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirPrefixOpr {
    Minus,
    NotBool,
    NotInt,
    BitNot,
    TakeRef,
    Deref,
}

impl HirPrefixOpr {
    pub fn from_sema(
        opr: SemaPrefixOpr,
        opd_ty: FluffyTerm,
        db: &::salsa::Db,
        fluffy_terms: &FluffyTerms,
    ) -> Self {
        match opr {
            SemaPrefixOpr::Minus => HirPrefixOpr::Minus,
            SemaPrefixOpr::Not => match opd_ty.base_ty_data_inner(db, fluffy_terms) {
                husky_fluffy_term::FluffyBaseTypeData::TypeOntology {
                    ty_path: _,
                    refined_ty_path: Left(prelude_ty_path),
                    ty_arguments: _,
                    ty_ethereal_term: _,
                } => match prelude_ty_path {
                    PreludeTypePath::BOOL => HirPrefixOpr::NotBool,
                    PreludeTypePath::Num(PreludeNumTypePath::Int(_)) => HirPrefixOpr::NotInt,
                    _ => todo!(),
                },
                _ => todo!(),
            },
            SemaPrefixOpr::BitNot => HirPrefixOpr::BitNot,
            SemaPrefixOpr::LeashType | SemaPrefixOpr::RefType | SemaPrefixOpr::Option => {
                unreachable!()
            }
        }
    }
}
