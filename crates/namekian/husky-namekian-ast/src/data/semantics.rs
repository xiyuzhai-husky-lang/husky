use super::*;

#[salsa::tracked]
pub struct NamSemanticsAst {
    #[id]
    id: (),
}
