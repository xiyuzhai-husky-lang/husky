// use crate::*;

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct TermUniverse {
//     kind: TermUniverseKind,
//     level: TermUniverseLevel,
// }

// impl Into<Term> for TermUniverse {
//     fn into(self) -> Term {
//         todo!()
//         // Term::Universe(self)
//     }
// }

// impl TermUniverse {
//     pub fn kind(&self) -> TermUniverseKind {
//         self.kind
//     }

//     pub fn level(&self) -> TermUniverseLevel {
//         self.level
//     }

//     // pub(crate) fn ty_universe(&self) -> TermUniverse {
//     //     TermUniverse {
//     //         kind: UniverseKind::Type,
//     //         level: self.level.next().expect("todo"),
//     //     }
//     // }

//     pub fn zeroth_ty_universe() -> TermUniverse {
//         Self {
//             kind: TermUniverseKind::Type,
//             level: TermUniverseLevel::zero(),
//         }
//     }

//     pub fn ty_universe(level: TermUniverseLevel) -> Term {
//         TermUniverse {
//             kind: TermUniverseKind::Type,
//             level,
//         }
//         .into()
//     }
// }

// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// pub enum TermUniverseKind {
//     Type,
//     Sort,
//     Term,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct TermUniverseLevel(u8);

// const UNIVERSE_MAX: u8 = 100;

// impl TermUniverseLevel {
//     pub(crate) fn zero() -> Self {
//         TermUniverseLevel(0)
//     }

//     pub(crate) fn next(self) -> TermResult<Self> {
//         if !(self.0 < UNIVERSE_MAX) {
//             return Err(TermError::UniverseOverflow);
//         }
//         Ok(TermUniverseLevel(self.0 + 1))
//     }

//     // pub(crate) fn prev(self) -> Option<Self> {
//     //     if self.0 == 0 {
//     //         return None;
//     //     }
//     //     Some(Universe(self.0 - 1))
//     // }

//     pub(crate) fn max(self, other: TermUniverseLevel) -> TermUniverseLevel {
//         TermUniverseLevel(self.0.max(other.0))
//     }

//     pub(crate) fn positive(self) -> bool {
//         self.0 > 0
//     }
// }
// impl std::fmt::Display for TermUniverse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
