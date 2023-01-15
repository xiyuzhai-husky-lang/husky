use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermUniverse(u8);

impl From<TermUniverse> for TermAtom {
    fn from(val: TermUniverse) -> Self {
        TermAtom::Universe(val)
    }
}

impl From<TermUniverse> for TermData {
    fn from(val: TermUniverse) -> Self {
        TermData::Atom(val.into())
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TermData {
    pub(crate) fn as_universe(self: &TermData) -> TermResult<TermUniverse> {
        match self {
            TermData::Atom(a) => match a {
                TermAtom::Literal(_) => todo!(),
                TermAtom::Variable {
                    variable_variant: _,
                } => todo!(),
                TermAtom::Entity { .. } => todo!(),
                TermAtom::Category(_category_kind) => todo!(),
                TermAtom::Universe(u) => Ok(*u),
            },
            TermData::Curry(_) => todo!(),
            TermData::Abstraction(_) => todo!(),
            TermData::Application(_) => todo!(),
            TermData::Subentity(_) => todo!(),
            TermData::TraitImpl(_) => todo!(),
        }
    }
}
