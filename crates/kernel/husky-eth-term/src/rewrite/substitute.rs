use super::*;
use crate::term::hvar::EthHvar;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EthTermSubstitution {
    src: EthHvar,
    dst: EthTerm,
}

impl EthTermSubstitution {
    pub fn new(src: EthHvar, dst: EthTerm) -> Self {
        Self { src, dst }
    }
}

/// # getters
impl EthTermSubstitution {
    pub fn src(&self) -> EthHvar {
        self.src
    }

    pub fn dst(&self) -> EthTerm {
        self.dst
    }
}

pub trait EthTermSubstitute<'a>: Copy {
    type Output;

    fn substitute(self, substitution: EthTermSubstitution, db: &'a ::salsa::Db) -> Self::Output;
}

impl<'a, T> EthTermSubstitute<'a> for &'a [T]
where
    T: EthTermSubstitute<'a>,
{
    type Output = impl Iterator<Item = T::Output> + 'a;

    fn substitute(self, substitution: EthTermSubstitution, db: &'a salsa::Db) -> Self::Output {
        self.iter()
            .copied()
            .map(move |elem| elem.substitute(substitution, db))
    }
}
