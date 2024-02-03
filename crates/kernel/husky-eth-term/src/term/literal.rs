use super::*;

impl EthTerm {
    pub(super) fn from_literal_declarative_term(
        db: &::salsa::Db,
        literal: DecLiteral,
        ty_expectation: TypeFinalDestinationExpectation,
    ) -> EthTermResult<Self> {
        Ok(match literal {
            DecLiteral::Resolved(literal) => literal.into(),
            DecLiteral::Unresolved(literal) => {
                Self::from_unresolved_literal_declarative_term(literal, ty_expectation, db)
            }
        })
    }

    fn from_unresolved_literal_declarative_term(
        literal: UnresolvedDecLiteral,
        ty_expectation: TypeFinalDestinationExpectation,
        db: &salsa::Db,
    ) -> EthTerm {
        match literal {
            UnresolvedDecLiteral::RegularInteger(i) => {
                let TypeFinalDestinationExpectation::EqsNonSortTypePath(ty_path) = ty_expectation
                else {
                    todo!()
                };
                let Some(PreludeTypePath::Num(PreludeNumTypePath::Int(int_ty_path))) =
                    ty_path.prelude_ty_path(db)
                else {
                    todo!()
                };
                Literal::from_unspecified_int(int_ty_path, i, db).into()
            }
        }
    }
}
