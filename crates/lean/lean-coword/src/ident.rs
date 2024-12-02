use coword::Coword;
use eterned::db::EternerDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LnIdent(Coword);

impl LnIdent {
    pub fn from_owned(ident: String, db: &EternerDb) -> Self {
        LnIdent(Coword::new(ident, db))
    }

    pub fn from_ref(ident: &str, db: &EternerDb) -> Self {
        // TODO: allow broader idents, like h\1
        assert!(Self::is_valid(ident));
        LnIdent(Coword::from_ref(ident, db))
    }

    pub fn is_valid(ident: &str) -> bool {
        // Check if the identifier is valid according to Lean rules
        // This is a placeholder implementation; adjust as needed
        ident.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    pub fn is_reserved(ident: &str) -> bool {
        // Check if the identifier is a reserved keyword in Lean
        // This list is not exhaustive; add more keywords as needed
        const RESERVED_KEYWORDS: &[&str] = &[
            "def",
            "theorem",
            "axiom",
            "variable",
            "import",
            "inductive",
            "structure",
            "class",
            "instance",
            "where",
            "let",
            "in",
            "if",
            "then",
            "else",
            "match",
        ];
        RESERVED_KEYWORDS.contains(&ident)
    }

    pub fn data(self, db: &EternerDb) -> &str {
        self.0.data(db)
    }
}
