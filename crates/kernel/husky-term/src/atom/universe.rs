use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermUniverse(u8);

impl Into<TermAtom> for TermUniverse {
    fn into(self) -> TermAtom {
        TermAtom::Universe(self)
    }
}

impl Into<TermOwned> for TermUniverse {
    fn into(self) -> TermOwned {
        TermOwned::Atom(self.into())
    }
}

const UNIVERSE_MAX: u8 = 100;

impl TermUniverse {
    pub(crate) fn new(i: u8) -> Self {
        assert!(i < UNIVERSE_MAX);
        TermUniverse(i)
    }

    pub(crate) fn zero() -> Self {
        TermUniverse(0)
    }

    pub(crate) fn next(self) -> TermResult<Self> {
        if !(self.0 < UNIVERSE_MAX) {
            return Err(TermError::UniverseOverflow);
        }
        Ok(TermUniverse(self.0 + 1))
    }

    // pub(crate) fn prev(self) -> Option<Self> {
    //     if self.0 == 0 {
    //         return None;
    //     }
    //     Some(Universe(self.0 - 1))
    // }

    pub(crate) fn max(self, other: TermUniverse) -> TermUniverse {
        TermUniverse(self.0.max(other.0))
    }

    pub(crate) fn positive(self) -> bool {
        self.0 > 0
    }
}

impl std::fmt::Display for TermUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TermOwned {
    pub(crate) fn as_universe(self: &TermOwned) -> TermResult<TermUniverse> {
        match self {
            TermOwned::Atom(a) => match a {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable {
                    ref variable_variant,
                } => todo!(),
                TermAtom::Entity { .. } => todo!(),
                TermAtom::Category(category_kind) => todo!(),
                TermAtom::Universe(u) => Ok(*u),
            },
            TermOwned::Curry(_) => todo!(),
            TermOwned::Abstraction(_) => todo!(),
            TermOwned::Application(_) => todo!(),
            TermOwned::Subentity(_) => todo!(),
            TermOwned::TraitImpl(_) => todo!(),
        }
    }
}
