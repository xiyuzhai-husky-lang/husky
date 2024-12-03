use husky_unicode_symbols::greek::is_greek;

use crate::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

/// # constructors

impl Label {
    pub fn from_owned(db: &::salsa::Db, data: String) -> Option<Self> {
        Some(Self {
            kind: LabelKind::new(&data),
            ident: Ident::from_owned(db, data)?,
        })
    }

    pub fn from_ref(db: &::salsa::Db, data: &str) -> Option<Self> {
        Some(Self {
            kind: LabelKind::new(&data),
            ident: Ident::from_ref(db, data)?,
        })
    }
}

/// # getters

impl Label {
    pub fn data(&self) -> &str {
        self.ident.data()
    }

    pub fn coword(self) -> Coword {
        self.ident.coword()
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

impl std::fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`'{}`", &self.ident.data()))
    }
}

impl salsa::DebugWithDb for Label {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("`'{}`", &self.ident.data()))
    }
}
