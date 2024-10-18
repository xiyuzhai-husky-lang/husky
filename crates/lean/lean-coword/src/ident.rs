use husky_coword::Coword;
use salsa::db::Db;

pub struct LeanIdent(Coword);

impl LeanIdent {
    pub fn new(db: &Db, ident: &str) -> Self {
        // TODO: allow broader idents, like h\1
        assert!(Self::is_valid(ident));
        LeanIdent(Coword::from_ref(db, ident))
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
}
