use husky_entity_path::TypePath;
use vec_like::AsVecMapEntry;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct DeclarativeTermQualifiedTypeholderShowEntry {
    variable: DeclarativeTermQualifiedTypeholder,
    show_kind: DeclarativeTermQualifiedTypeholderShowKind,
    idx: u8,
    /// number of lambdas using this variable
    /// level 0 means this variable is external
    level: u8,
    external_variable_ident: Option<Ident>,
}

impl DeclarativeTermQualifiedTypeholderShowEntry {
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
                DeclarativeTermQualifiedTypeholderShowKind::Lifetime => match self.idx {
                    0 => f.write_str("'a"),
                    1 => f.write_str("'b"),
                    2 => f.write_str("'c"),
                    3 => f.write_str("'d"),
                    4 => f.write_str("'e"),
                    5 => f.write_str("'f"),
                    idx => f.write_fmt(format_args!("'a{}", idx)),
                },
                DeclarativeTermQualifiedTypeholderShowKind::Binding => match self.idx {
                    0 => f.write_str("'α"),
                    1 => f.write_str("'β"),
                    2 => f.write_str("'γ"),
                    3 => f.write_str("'δ"),
                    4 => f.write_str("'ϵ"),
                    5 => f.write_str("'ζ"),
                    6 => f.write_str("'η"),
                    idx => f.write_fmt(format_args!("'α{}", idx)),
                },
                DeclarativeTermQualifiedTypeholderShowKind::Prop => match self.idx {
                    0 => f.write_str("p"),
                    1 => f.write_str("q"),
                    idx => f.write_fmt(format_args!("p{}", idx)),
                },
                DeclarativeTermQualifiedTypeholderShowKind::Type => match self.idx {
                    0 => f.write_str("t"),
                    1 => f.write_str("s"),
                    idx => f.write_fmt(format_args!("t{}", idx)),
                },
                DeclarativeTermQualifiedTypeholderShowKind::Kind => match self.idx {
                    0 => f.write_str("α"),
                    1 => f.write_str("β"),
                    2 => f.write_str("γ"),
                    3 => f.write_str("δ"),
                    4 => f.write_str("ϵ"),
                    5 => f.write_str("ζ"),
                    6 => f.write_str("η"),
                    idx => f.write_fmt(format_args!("α{}", idx)),
                },
                DeclarativeTermQualifiedTypeholderShowKind::Other => match self.idx {
                    0 => f.write_str("a"),
                    1 => f.write_str("b"),
                    idx => f.write_fmt(format_args!("a{}", idx)),
                },
            }
        }
    }
}

impl AsVecMapEntry for DeclarativeTermQualifiedTypeholderShowEntry {
    type K = DeclarativeTermQualifiedTypeholder;

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
pub(crate) enum DeclarativeTermQualifiedTypeholderShowKind {
    Lifetime,
    Binding,
    Prop,
    Type,
    Kind,
    Other,
}

impl DeclarativeTermShowContext {
    pub(super) fn new_external_entry(
        &self,
        db: &::salsa::Db,
        variable: DeclarativeTermQualifiedTypeholder,
        external_variable_ident: Option<Ident>,
    ) -> DeclarativeTermQualifiedTypeholderShowEntry {
        self.new_entry(db, variable, 0, external_variable_ident)
    }

    pub(super) fn new_internal_entry(
        &self,
        db: &::salsa::Db,
        variable: DeclarativeTermQualifiedTypeholder,
    ) -> DeclarativeTermQualifiedTypeholderShowEntry {
        self.new_entry(db, variable, 1, None)
    }

    fn new_entry(
        &self,
        db: &::salsa::Db,
        variable: DeclarativeTermQualifiedTypeholder,
        level: u8,
        external_variable_ident: Option<Ident>,
    ) -> DeclarativeTermQualifiedTypeholderShowEntry {
        let show_kind = variable_show_kind(variable, db);
        let idx = self.issue_idx(show_kind);
        DeclarativeTermQualifiedTypeholderShowEntry {
            variable,
            show_kind,
            idx,
            level,
            external_variable_ident,
        }
    }

    fn issue_idx(&self, show_kind: DeclarativeTermQualifiedTypeholderShowKind) -> u8 {
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
    pub(super) fn with_variable(
        &mut self,
        db: &::salsa::Db,
        variable: DeclarativeTermQualifiedTypeholder,
    ) {
        if let Some(entry) = self.entries.get_entry_mut(variable) {
            entry.level += 1
        } else {
            let new_entry = self.new_internal_entry(db, variable);
            self.entries.insert_new(new_entry).unwrap();
        }
    }

    pub(super) fn without_variable(&mut self, variable: DeclarativeTermQualifiedTypeholder) {
        self.entries.get_entry_mut(variable).unwrap().level -= 1
    }
}

fn variable_show_kind(
    variable: DeclarativeTermQualifiedTypeholder,
    db: &::salsa::Db,
) -> DeclarativeTermQualifiedTypeholderShowKind {
    match variable.ty(db) {
        Ok(DeclarativeTerm::EntityPath(DeclarativeTermEntityPath::Type(ty)))
            if ty.eqs_lifetime_ty_path(db) =>
        {
            DeclarativeTermQualifiedTypeholderShowKind::Lifetime
        }
        Ok(DeclarativeTerm::Category(cat)) if cat.universe().raw() == 0 => {
            DeclarativeTermQualifiedTypeholderShowKind::Prop
        }
        Ok(DeclarativeTerm::Category(cat)) if cat.universe().raw() == 1 => {
            DeclarativeTermQualifiedTypeholderShowKind::Type
        }
        Ok(DeclarativeTerm::Category(_)) => DeclarativeTermQualifiedTypeholderShowKind::Kind,
        _ => DeclarativeTermQualifiedTypeholderShowKind::Other,
    }
}
