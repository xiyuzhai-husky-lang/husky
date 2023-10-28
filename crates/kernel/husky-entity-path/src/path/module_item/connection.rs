use husky_coword::IdentPairMap;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MajorItemConnection {
    Connected,
    Disconnected(Disambiguator),
}
impl MajorItemConnection {
    pub(crate) fn kind(&self) -> MajorItemConnectionKind {
        match self {
            MajorItemConnection::Connected => MajorItemConnectionKind::Connected,
            MajorItemConnection::Disconnected(_) => MajorItemConnectionKind::Disconnected,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Disambiguator(u8);

#[derive(Debug, Default)]
pub struct DisconnectedConnectionRegistry {
    next_raws: IdentPairMap<u8>,
}

impl DisconnectedConnectionRegistry {
    pub fn issue_new(&mut self, ident: Ident) -> Disambiguator {
        let next_raw = self.next_raws.get_value_mut_or_insert_default(ident);
        let disambiguator = Disambiguator(*next_raw);
        *next_raw += 1;
        disambiguator
    }
}
