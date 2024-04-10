use super::*;

#[salsa::tracked]
pub struct NamStatementAst {
    #[id]
    id: (),
}
