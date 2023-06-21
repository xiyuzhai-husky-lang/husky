use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct EnumTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: EnumTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = pub(super) new)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}
