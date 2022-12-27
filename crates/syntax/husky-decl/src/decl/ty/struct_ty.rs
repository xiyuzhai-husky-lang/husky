use husky_expr::ExprIdx;
use husky_word::Identifier;

use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct StructTypeDecl {
    pub module_item_path: ModuleItemPath,
    #[return_ref]
    pub fields: Vec<FieldDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldDecl {
    ident: Identifier,
    ty: ExprIdx,
}
