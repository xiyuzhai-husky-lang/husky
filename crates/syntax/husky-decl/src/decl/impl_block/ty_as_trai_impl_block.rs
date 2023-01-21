use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitImplBlockDecl {
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}
