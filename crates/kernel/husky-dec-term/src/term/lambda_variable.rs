mod r#abstract;
mod index;
mod set;

pub use self::set::*;

use super::*;
use crate::helpers::DecTermFamily;

/// variables are externalized symbols, derived from symbols, and defined in a bottom-up manner
///
#[salsa::interned(constructor = new_inner)]
pub struct DecLambdaVariable {
    pub ty: DecTermSymbolicVariableTypeResult<DecTerm>,
    /// this is the index to disambiguate it from all other symbols with the same type
    /// so that we have better cache hits
    /// todo: change to RefinedDeBrujinIndex
    pub index: LambdaVariableIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LambdaVariableIndex {
    ty_family: DecTermFamily,
    disambiguator: u8,
}

impl LambdaVariableIndex {
    pub fn ty_family(self) -> DecTermFamily {
        self.ty_family
    }

    pub fn disambiguator(self) -> u8 {
        self.disambiguator
    }
}

impl std::fmt::Display for LambdaVariableIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.disambiguator, f)
    }
}

impl salsa::DisplayWithDb for DecLambdaVariable {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        self.index(db).display_fmt_with_db(f, db)
    }
}

impl DecLambdaVariable {
    pub fn new(
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        disambiguator: u8,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(
            db,
            ty,
            LambdaVariableIndex {
                ty_family: match ty {
                    Ok(ty) => ty.family(db),
                    Err(_) => DecTermFamily::Other,
                },
                disambiguator,
            },
        )
    }
}

impl DecTermRewriteCopy for DecLambdaVariable {
    fn substitute_copy(self, _db: &::salsa::Db, _substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}
