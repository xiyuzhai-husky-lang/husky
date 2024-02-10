use crate::*;
use husky_term_prelude::symbol::SymbolName;
use vec_like::VecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DecSvarNameMap {
    data: VecPairMap<DecSvar, SymbolName>,
}

impl DecSvarNameMap {
    pub fn add(&mut self, symbol: DecSvar, name: SymbolName) {
        self.data.insert((symbol, name))
    }

    pub fn data(&self) -> &VecPairMap<DecSvar, SymbolName> {
        &self.data
    }
}

impl std::ops::Index<DecSvar> for DecSvarNameMap {
    type Output = SymbolName;

    fn index(&self, index: DecSvar) -> &Self::Output {
        &self.data[index].1
    }
}

pub struct DecTermWithNameMap<'a> {
    term: DecTerm,
    name_map: &'a DecSvarNameMap,
}

impl<'a> salsa::DisplayWithDb for DecTermWithNameMap<'a> {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        self.term.display_fmt_with_db_and_ctx(f, db, self.name_map)
    }
}

impl<'a> salsa::DebugWithDb for DecTermWithNameMap<'a> {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;

        f.write_str("DecTerm(`")?;
        self.display_fmt_with_db(f, db)?;
        f.write_str("`)")
    }
}

impl DecTerm {
    pub fn with_symbol_source_map<'a>(
        self,
        name_map: &'a DecSvarNameMap,
    ) -> DecTermWithNameMap<'a> {
        DecTermWithNameMap {
            term: self,
            name_map,
        }
    }
}
