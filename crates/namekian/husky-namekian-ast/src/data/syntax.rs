use super::*;

#[salsa::tracked]
pub struct NamSyntaxAst {
    #[id]
    id: (),
}
