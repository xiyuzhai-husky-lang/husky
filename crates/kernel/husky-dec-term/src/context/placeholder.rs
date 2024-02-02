use husky_entity_path::TypePath;
use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct DecTermQualifiedTypeholderShowEntry {
    variable: DecTermQualifiedTypeholder,
    show_kind: DecTermQualifiedTypeholderShowKind,
    idx: u8,
    /// number of lambdas using this variable
    /// level 0 means this variable is external
    level: u8,
    external_variable_ident: Option<Ident>,
}

impl DecTermQualifiedTypeholderShowEntry {
    pub(crate) fn show(
        &self,
        _db: &::salsa::Db,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(_external_variable_ident) = self.external_variable_ident
            && self.level == 0
        {
            todo!()
        } else {
            match self.show_kind {
                DecTermQualifiedTypeholderShowKind::Lifetime => match self.idx {
                    0 => f.write_str("'a"),
                    1 => f.write_str("'b"),
                    2 => f.write_str("'c"),
                    3 => f.write_str("'d"),
                    4 => f.write_str("'e"),
                    5 => f.write_str("'f"),
                    idx => f.write_fmt(format_args!("'a{}", idx)),
                },
                DecTermQualifiedTypeholderShowKind::Binding => match self.idx {
                    0 => f.write_str("'α"),
                    1 => f.write_str("'β"),
                    2 => f.write_str("'γ"),
                    3 => f.write_str("'δ"),
                    4 => f.write_str("'ϵ"),
                    5 => f.write_str("'ζ"),
                    6 => f.write_str("'η"),
                    idx => f.write_fmt(format_args!("'α{}", idx)),
                },
                DecTermQualifiedTypeholderShowKind::Prop => match self.idx {
                    0 => f.write_str("p"),
                    1 => f.write_str("q"),
                    idx => f.write_fmt(format_args!("p{}", idx)),
                },
                DecTermQualifiedTypeholderShowKind::Type => match self.idx {
                    0 => f.write_str("t"),
                    1 => f.write_str("s"),
                    idx => f.write_fmt(format_args!("t{}", idx)),
                },
                DecTermQualifiedTypeholderShowKind::Kind => match self.idx {
                    0 => f.write_str("α"),
                    1 => f.write_str("β"),
                    2 => f.write_str("γ"),
                    3 => f.write_str("δ"),
                    4 => f.write_str("ϵ"),
                    5 => f.write_str("ζ"),
                    6 => f.write_str("η"),
                    idx => f.write_fmt(format_args!("α{}", idx)),
                },
                DecTermQualifiedTypeholderShowKind::Other => match self.idx {
                    0 => f.write_str("a"),
                    1 => f.write_str("b"),
                    idx => f.write_fmt(format_args!("a{}", idx)),
                },
            }
        }
    }
}

impl AsVecMapEntry for DecTermQualifiedTypeholderShowEntry {
    type K = DecTermQualifiedTypeholder;

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
pub(crate) enum DecTermQualifiedTypeholderShowKind {
    Lifetime,
    Binding,
    Prop,
    Type,
    Kind,
    Other,
}

impl DecTermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &::salsa::Db,
        variable: DecTermQualifiedTypeholder,
        external_variable_ident: Option<Ident>,
    ) -> DecTermQualifiedTypeholderShowEntry {
        self.new_entry(db, variable, 0, external_variable_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &::salsa::Db,
        variable: DecTermQualifiedTypeholder,
    ) -> DecTermQualifiedTypeholderShowEntry {
        self.new_entry(db, variable, 1, None)
    }

    fn new_entry(
        &self,
        db: &::salsa::Db,
        variable: DecTermQualifiedTypeholder,
        level: u8,
        external_variable_ident: Option<Ident>,
    ) -> DecTermQualifiedTypeholderShowEntry {
        let show_kind = variable_show_kind(variable, db);
        let idx = self.issue_idx(show_kind);
        DecTermQualifiedTypeholderShowEntry {
            variable,
            show_kind,
            idx,
            level,
            external_variable_ident,
        }
    }

    fn issue_idx(&self, show_kind: DecTermQualifiedTypeholderShowKind) -> u8 {
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
    pub(super) fn with_variable(&mut self, db: &::salsa::Db, variable: DecTermQualifiedTypeholder) {
        if let Some(entry) = self.entries.get_entry_mut(variable) {
            entry.level += 1
        } else {
            let new_entry = self.new_internal_entry(db, variable);
            self.entries.insert_new(new_entry).unwrap();
        }
    }

    pub(super) fn without_variable(&mut self, variable: DecTermQualifiedTypeholder) {
        self.entries.get_entry_mut(variable).unwrap().level -= 1
    }
}

fn variable_show_kind(
    variable: DecTermQualifiedTypeholder,
    db: &::salsa::Db,
) -> DecTermQualifiedTypeholderShowKind {
    match variable.ty(db) {
        Ok(DecTerm::EntityPath(DecTermEntityPath::Type(ty))) if ty.eqs_lifetime_ty_path(db) => {
            DecTermQualifiedTypeholderShowKind::Lifetime
        }
        Ok(DecTerm::Category(cat)) if cat.universe().raw() == 0 => {
            DecTermQualifiedTypeholderShowKind::Prop
        }
        Ok(DecTerm::Category(cat)) if cat.universe().raw() == 1 => {
            DecTermQualifiedTypeholderShowKind::Type
        }
        Ok(DecTerm::Category(_)) => DecTermQualifiedTypeholderShowKind::Kind,
        _ => DecTermQualifiedTypeholderShowKind::Other,
    }
}
