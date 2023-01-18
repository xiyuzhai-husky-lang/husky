use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct FeatureSignature {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
}
