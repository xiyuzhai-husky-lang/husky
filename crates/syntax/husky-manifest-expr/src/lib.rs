use husky_doc::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ManifestExpr {
    pub variant: ManifestExprVariant,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ManifestExprVariant {
    Equals { name: String },
}
