use husky_entity_path::TypePath;
use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct RawTermPlaceholderShowEntry {
    variable: RawTermPlaceholder,
    show_kind: RawTermPlaceholderShowKind,
    idx: u8,
    /// number of lambdas using this variable
    /// level 0 means this variable is external
    level: u8,
    external_variable_ident: Option<Ident>,
}

impl RawTermPlaceholderShowEntry {
    pub(crate) fn show(
        &self,
        _db: &dyn RawTermDb,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(_external_variable_ident) = self.external_variable_ident && self.level == 0 {
            todo!()
        } else {
            match self.show_kind {
                RawTermPlaceholderShowKind::Lifetime => {
                    match self.idx {
                        0 => f.write_str("'a"),
                        1 => f.write_str("'b"),
                        2 => f.write_str("'c"),
                        3 => f.write_str("'d"),
                        4 => f.write_str("'e"),
                        5 => f.write_str("'f"),
                        idx => f.write_fmt(format_args!("'a{}", idx))
                    }
                },
                RawTermPlaceholderShowKind::Binding => {
                    match self.idx {
                        0 => f.write_str("'α"),
                        1 => f.write_str("'β"),
                        2 => f.write_str("'γ"),
                        3 => f.write_str("'δ"),
                        4 => f.write_str("'ϵ"),
                        5 => f.write_str("'ζ"),
                        6 => f.write_str("'η"),
                        idx => f.write_fmt(format_args!("'α{}", idx))
                    }
                },
                RawTermPlaceholderShowKind::Prop => {
                    match self.idx {
                        0 => f.write_str("p"),
                        1 => f.write_str("q"),
                        idx => f.write_fmt(format_args!("p{}", idx))
                    }
                },
                RawTermPlaceholderShowKind::Type => {
                    match self.idx {
                        0 => f.write_str("t"),
                        1 => f.write_str("s"),
                        idx => f.write_fmt(format_args!("t{}", idx))
                    }
                },
                RawTermPlaceholderShowKind::Kind => {
                    match self.idx {
                        0 => f.write_str("α"),
                        1 => f.write_str("β"),
                        2 => f.write_str("γ"),
                        3 => f.write_str("δ"),
                        4 => f.write_str("ϵ"),
                        5 => f.write_str("ζ"),
                        6 => f.write_str("η"),
                        idx => f.write_fmt(format_args!("α{}", idx))
                    }
                },
                RawTermPlaceholderShowKind::Other => {
                    match self.idx {
                        0 => f.write_str("a"),
                        1 => f.write_str("b"),
                        idx => f.write_fmt(format_args!("a{}", idx))
                    }
                }
            }
        }
    }
}

impl AsVecMapEntry for RawTermPlaceholderShowEntry {
    type K = RawTermPlaceholder;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.variable
    }

    fn key_ref(&self) -> &Self::K {
        &self.variable
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum RawTermPlaceholderShowKind {
    Lifetime,
    Binding,
    Prop,
    Type,
    Kind,
    Other,
}

impl RawTermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &dyn RawTermDb,
        variable: RawTermPlaceholder,
        external_variable_ident: Option<Ident>,
    ) -> RawTermPlaceholderShowEntry {
        self.new_entry(db, variable, 0, external_variable_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &dyn RawTermDb,
        variable: RawTermPlaceholder,
    ) -> RawTermPlaceholderShowEntry {
        self.new_entry(db, variable, 1, None)
    }

    fn new_entry(
        &self,
        db: &dyn RawTermDb,
        variable: RawTermPlaceholder,
        level: u8,
        external_variable_ident: Option<Ident>,
    ) -> RawTermPlaceholderShowEntry {
        let show_kind = variable_show_kind(variable, db);
        let idx = self.issue_idx(show_kind);
        RawTermPlaceholderShowEntry {
            variable,
            show_kind,
            idx,
            level,
            external_variable_ident,
        }
    }

    fn issue_idx(&self, show_kind: RawTermPlaceholderShowKind) -> u8 {
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

    // todo: put this into an internal table struct
    pub(super) fn with_variable(&mut self, db: &dyn RawTermDb, variable: RawTermPlaceholder) {
        if let Some(entry) = self.entries.get_entry_mut(variable) {
            entry.level += 1
        } else {
            let new_entry = self.new_internal_entry(db, variable);
            self.entries.insert_new(new_entry).unwrap();
        }
    }

    pub(super) fn without_variable(&mut self, variable: RawTermPlaceholder) {
        self.entries.get_entry_mut(variable).unwrap().level -= 1
    }
}

fn variable_show_kind(
    variable: RawTermPlaceholder,
    db: &dyn RawTermDb,
) -> RawTermPlaceholderShowKind {
    match variable.ty(db) {
        Ok(RawTerm::EntityPath(RawTermEntityPath::Type(ty))) if ty.eqs_lifetime_ty_path(db) => {
            RawTermPlaceholderShowKind::Lifetime
        }
        Ok(RawTerm::Category(cat)) if cat.universe().raw() == 0 => RawTermPlaceholderShowKind::Prop,
        Ok(RawTerm::Category(cat)) if cat.universe().raw() == 1 => RawTermPlaceholderShowKind::Type,
        Ok(RawTerm::Category(_)) => RawTermPlaceholderShowKind::Kind,
        _ => RawTermPlaceholderShowKind::Other,
    }
}
