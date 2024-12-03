pub mod literal;

use self::literal::{LnLiteral, LnLiteralData};
use eterned::db::EternerDb;
use lean_entity_path::LnItemPath;

// TODO: ad hoc, use LnTermId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LnTerm {
    Literal(LnLiteral),
    ItemPath(LnItemPath),
}

impl std::fmt::Debug for LnTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl LnTerm {
    pub fn new_item_path(path: LnItemPath) -> Self {
        LnTerm::ItemPath(path)
    }
}

#[eterned::eterned]
pub struct LnTermId {
    #[return_ref]
    data: LnTermData,
}

impl std::fmt::Debug for LnTermId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.debug_tuple("LnTermId").field(&self.0).finish()
    }
}

#[enum_class::from_variants]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LnTermData {
    Literal(LnLiteralData),
}

impl LnTerm {
    pub fn show(&self, db: &EternerDb) -> String {
        match self {
            LnTerm::Literal(literal) => literal.show(db),
            LnTerm::ItemPath(item_path) => item_path.show(db),
        }
    }
}
