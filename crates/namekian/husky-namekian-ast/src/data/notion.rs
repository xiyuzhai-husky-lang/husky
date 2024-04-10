use super::*;

#[salsa::tracked]
pub struct NamNotionAst {
    #[id]
    id: (),
}
