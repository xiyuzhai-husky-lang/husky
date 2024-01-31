use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EtherealTermSubstitution {
    src: RuneEtherealTerm,
    dst: EtherealTerm,
}

impl EtherealTermSubstitution {
    pub fn new(src: RuneEtherealTerm, dst: EtherealTerm) -> Self {
        Self { src, dst }
    }
}

/// # getters
impl EtherealTermSubstitution {
    pub fn src(&self) -> RuneEtherealTerm {
        self.src
    }

    pub fn dst(&self) -> EtherealTerm {
        self.dst
    }
}

pub trait EtherealTermSubstitute<'a>: Copy {
    type Output;

    fn substitute(
        self,
        substitution: EtherealTermSubstitution,
        db: &'a ::salsa::Db,
    ) -> Self::Output;
}

impl<'a, T> EtherealTermSubstitute<'a> for &'a [T]
where
    T: EtherealTermSubstitute<'a>,
{
    type Output = impl Iterator<Item = T::Output> + 'a;

    fn substitute(self, substitution: EtherealTermSubstitution, db: &'a salsa::Db) -> Self::Output {
        self.iter()
            .copied()
            .map(move |elem| elem.substitute(substitution, db))
    }
}
