use husky_entity_path::ModuleItemPath;
use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct TermSymbolShowEntry {
    symbol: TermSymbol,
    show_kind: TermSymbolShowKind,
    idx: u8,
    /// number of lambdas using this symbol
    /// level 0 means this symbol is external
    level: u8,
    external_symbol_ident: Option<Identifier>,
}

impl AsVecMapEntry for TermSymbolShowEntry {
    type K = TermSymbol;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.symbol
    }

    fn key_ref(&self) -> &Self::K {
        &self.symbol
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum TermSymbolShowKind {
    Lifetime,
    Type,
    OtherSort,
    Other,
}

impl TermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &dyn TermDb,
        symbol: TermSymbol,
        external_symbol_ident: Option<Identifier>,
    ) -> TermSymbolShowEntry {
        self.new_entry(db, symbol, 0, external_symbol_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &dyn TermDb,
        symbol: TermSymbol,
    ) -> TermSymbolShowEntry {
        self.new_entry(db, symbol, 1, None)
    }

    fn new_entry(
        &self,
        db: &dyn TermDb,
        symbol: TermSymbol,
        level: u8,
        external_symbol_ident: Option<Identifier>,
    ) -> TermSymbolShowEntry {
        let show_kind = match symbol.ty(db) {
            Ok(Term::Entity(EntityPath::ModuleItem(ModuleItemPath::Type(ty))))
                if ty.is_lifetime_ty(db) =>
            {
                TermSymbolShowKind::Lifetime
            }
            Ok(Term::Category(cat)) if cat.universe().raw() == 0 => TermSymbolShowKind::Type,
            Ok(Term::Category(_)) => TermSymbolShowKind::OtherSort,
            _ => TermSymbolShowKind::Other,
        };
        let idx = self.issue_idx(show_kind);
        TermSymbolShowEntry {
            symbol,
            show_kind,
            idx,
            level,
            external_symbol_ident,
        }
    }

    fn issue_idx(&self, show_kind: TermSymbolShowKind) -> u8 {
        let last_idx = self
            .entries
            .data()
            .iter()
            .rev()
            .find(|entry| entry.show_kind == show_kind)
            .map(|entry| entry.idx);
        match last_idx {
            Some(last_idx) => last_idx + 1,
            None => 0,
        }
    }
}
