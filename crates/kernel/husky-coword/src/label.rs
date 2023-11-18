use husky_unicode_symbols::greek::is_greek;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Label {
    ident: Ident,
    kind: LabelKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LabelKind {
    AllGreek,
    AllNonGreek,
    Mixed,
}

impl LabelKind {
    fn new(s: &str) -> Self {
        let mut has_greek = false;
        let mut has_non_greek = false;
        for c in s.chars() {
            if is_greek(c) {
                has_greek = true
            } else {
                has_non_greek = true
            }
        }
        match (has_greek, has_non_greek) {
            (true, false) => LabelKind::AllGreek,
            (false, true) => LabelKind::AllNonGreek,
            (true, true) => LabelKind::Mixed,
            (false, false) => unreachable!(),
        }
    }
}

impl Label {
    pub fn word(self) -> Coword {
        self.ident.coword()
    }

    pub fn from_owned(db: &dyn CowordDb, data: String) -> Option<Self> {
        Some(Self {
            kind: LabelKind::new(&data),
            ident: Ident::from_owned(db, data)?,
        })
    }

    pub fn from_borrowed(db: &dyn CowordDb, data: &str) -> Option<Self> {
        Some(Self {
            kind: LabelKind::new(&data),
            ident: Ident::from_borrowed(db, data)?,
        })
    }

    pub fn is_valid_place_label(self) -> bool {
        self.kind == LabelKind::AllGreek
    }

    pub fn is_valid_lifetime_label(self) -> bool {
        self.kind == LabelKind::AllNonGreek
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }
}

impl<Db: CowordDb + ?Sized> salsa::DebugWithDb<Db> for Label {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<CowordJar>>::as_jar_db(db);
        if level.is_root() {
            f.debug_tuple("Label").field(&self.ident.data(db)).finish()
        } else {
            f.write_fmt(format_args!("`'{}`", &self.ident.data(db)))
        }
    }
}
