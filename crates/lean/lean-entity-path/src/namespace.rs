use crate::*;
use eterned::{
    db::{attached_interner_db, EternerDb},
    memo,
};
use lean_coword::ident::LnIdent;
use smallvec::{smallvec, SmallVec, ToSmallVec};

#[eterned::eterned(override_debug)]
pub struct LnNamespace {
    pub data: LnNamespaceData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LnNamespaceData {
    Root,
    Child(LnNamespace, LnIdent),
}

impl LnNamespace {
    pub fn new_root(db: &EternerDb) -> Self {
        Self::new(LnNamespaceData::Root, db)
    }

    pub fn from_ident_strs(idents: &[&str], db: &EternerDb) -> Self {
        let mut namespace = LnNamespace::new(LnNamespaceData::Root, db);
        for ident in idents {
            namespace = namespace.child(ident.to_string(), db);
        }
        namespace
    }

    pub fn child(self, ident: String, db: &EternerDb) -> Self {
        Self::new(
            LnNamespaceData::Child(self, LnIdent::from_owned(ident, db)),
            db,
        )
    }

    pub fn ident(self, db: &EternerDb) -> Option<LnIdent> {
        match self.data(db) {
            LnNamespaceData::Root => None,
            LnNamespaceData::Child(_, ident) => Some(ident),
        }
    }

    pub fn all_idents(self, db: &EternerDb) -> &[LnIdent] {
        ln_namespace_all_idents(self, db)
    }

    pub fn relative_idents(self, other: Self, db: &EternerDb) -> &[LnIdent] {
        let ids = self.all_idents(db);
        let other_ids = other.all_idents(db);
        let i = ids
            .iter()
            .zip(other_ids.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(ids.len().min(other_ids.len()));
        &ids[i..]
    }
}

impl std::fmt::Debug for LnNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = attached_interner_db().ok_or(std::fmt::Error)?;
        write!(
            f,
            "LnNamespace(`{}`)",
            self.all_idents(db)
                .iter()
                .map(|id| id.data(db))
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

#[memo(return_ref)]
fn ln_namespace_all_idents(namespace: LnNamespace, db: &EternerDb) -> SmallVec<[LnIdent; 4]> {
    match namespace.data(db) {
        LnNamespaceData::Root => smallvec![],
        LnNamespaceData::Child(parent, ident) => {
            let mut ids = parent.all_idents(db).to_smallvec();
            ids.push(ident);
            ids
        }
    }
}

#[test]
#[ignore]
fn ln_namespace_all_idents_works() {
    fn t(idents: &[&str]) {
        let db = &EternerDb::default();
        let namespace = LnNamespace::from_ident_strs(idents, db);
        let all_idents: Vec<&str> = namespace
            .all_idents(db)
            .iter()
            .map(|&ident| ident.data(db))
            .collect();
        assert_eq!(&all_idents as &[_], idents);
    }
    t(&["Root"]);
    t(&["Root", "Child"]);
    t(&["Root", "Child", "Grandchild"]);
}

#[test]
#[ignore]
fn ln_namespace_relative_idents_works() {
    fn t(slf: &[&str], other: &[&str], relative_idents: &[&str]) {
        let db = &EternerDb::default();
        let slf = LnNamespace::from_ident_strs(slf, db);
        let other = LnNamespace::from_ident_strs(other, db);
        assert_eq!(
            slf.relative_idents(other, db)
                .iter()
                .map(|&ident| ident.data(db))
                .collect::<Vec<_>>(),
            relative_idents,
            "slf: {slf:?}, other: {other:?}",
        );
    }
    t(&["Root"], &["Root"], &[]);
    t(&["Root"], &["Root", "Child"], &[]);
    t(&["Root", "Child"], &["Root", "Child", "Grandchild"], &[]);
    t(
        &["Root", "Child", "Grandchild"],
        &["Root", "Child"],
        &["Grandchild"],
    );
}
