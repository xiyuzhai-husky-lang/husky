use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermResolveProgress {
    Unresolved,
    Resolved {
        implicit_conversion: LocalTermImplicitConversion,
        local_term: LocalTerm,
    },
    Err(ExprTypeError),
}

impl LocalTermResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self) -> Self {
        match self {
            LocalTermResolveProgress::Unresolved => LocalTermResolveProgress::Unresolved,
            LocalTermResolveProgress::Resolved {
                implicit_conversion,
                local_term: term,
            } => LocalTermResolveProgress::Resolved {
                implicit_conversion: *implicit_conversion,
                local_term: *term,
            },
            LocalTermResolveProgress::Err(_) => {
                LocalTermResolveProgress::Err(DerivedExprTypeError::LocalTermResolveError.into())
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermResolveProgress::Unresolved => None,
            LocalTermResolveProgress::Resolved {
                local_term: term, ..
            } => match term {
                LocalTerm::Resolved(reduced_term) => Some(*reduced_term),
                LocalTerm::Unresolved(_) => todo!(),
            },
            LocalTermResolveProgress::Err(_) => None,
        }
    }
}
