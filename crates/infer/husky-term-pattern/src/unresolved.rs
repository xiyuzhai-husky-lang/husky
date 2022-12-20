use husky_entity_path::EntityPath;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnresolvedTermIdx(usize);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnresolvedTermRegistry {
    terms: Vec<UnresolvedTerm>,
}

#[derive(PartialEq, Eq)]
pub enum UnresolvedTerm {
    ImplicitArgument {
        scope: EntityPath,
        param_ident: Identifier,
    },
    IntegerLiteral(ExprIdx),
    IntegerType(UnresolvedTermIdx),
    FloatLiteral(ExprIdx),
    FloatType(UnresolvedTermIdx),
}

// HELP

impl std::fmt::Debug for UnresolvedTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImplicitArgument { scope: _, param_ident: _ } => {
                todo!()
                // write!(
                //     f,
                //     "ImplicitArgument {{ scope: {:?}, param_ident: \"{}\"}}",
                //     scope, param_ident
                // )
            }
            Self::IntegerLiteral(arg0) => {
                write!(f, "IntegerLiteral({:?})", arg0)
            }
            Self::IntegerType(arg0) => {
                write!(f, "IntegerType({:?})", arg0)
            }
            Self::FloatLiteral(arg0) => f.debug_tuple("FloatLiteral").field(arg0).finish(),
            Self::FloatType(arg0) => f.debug_tuple("FloatType").field(arg0).finish(),
        }
    }
}

impl UnresolvedTermRegistry {
    pub(crate) fn issue(&mut self, term: UnresolvedTerm) -> UnresolvedTermIdx {
        let raw = self.terms.len();
        self.terms.push(term);
        UnresolvedTermIdx(raw)
    }
}
