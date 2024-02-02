use super::*;

impl EthTerm {
    pub(super) fn from_literal_declarative_term(
        db: &::salsa::Db,
        literal: LiteralDecTerm,
        ty_expectation: TermTypeExpectation,
    ) -> EthTermResult<Self> {
        Ok(match literal {
            LiteralDecTerm::Resolved(literal) => literal.into(),
            LiteralDecTerm::Unresolved(literal) => {
                Self::from_unresolved_literal_declarative_term(literal, ty_expectation, db)
            }
        })
    }

    fn from_unresolved_literal_declarative_term(
        literal: UnresolvedTermLiteral,
        ty_expectation: TermTypeExpectation,
        db: &salsa::Db,
    ) -> EthTerm {
        match literal {
            UnresolvedTermLiteral::RegularInteger(i) => {
                let TermTypeExpectation::FinalDestinationEqsNonSortTypePath(ty_path) =
                    ty_expectation
                else {
                    todo!()
                };
                let Some(PreludeTypePath::Num(PreludeNumTypePath::Int(int_ty_path))) =
                    ty_path.prelude_ty_path(db)
                else {
                    todo!()
                };
                TermLiteral::from_unspecified_int(int_ty_path, i, db).into()
            }
        }
    }
}
