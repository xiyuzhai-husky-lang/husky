use husky_coword::IdentPairMap;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModuleItemConnection {
    Connected,
    Disconnected(Disambiguator),
}
impl ModuleItemConnection {
    pub(crate) fn kind(&self) -> ModuleItemConnectionKind {
        match self {
            ModuleItemConnection::Connected => ModuleItemConnectionKind::Connected,
            ModuleItemConnection::Disconnected(_) => ModuleItemConnectionKind::Disconnected,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Disambiguator(u8);

#[derive(Debug, Default)]
pub struct DisambiguatorRegistry {
    next_raws: IdentPairMap<u8>,
}

impl DisambiguatorRegistry {
    pub fn issue_new(&mut self, ident: Ident) -> Disambiguator {
        let next_raw = self.next_raws.get_value_mut_or_insert_default(ident);
        let disambiguator = Disambiguator(*next_raw);
        *next_raw += 1;
        disambiguator
    }
}
