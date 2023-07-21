use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct EnumTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: EnumTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = pub(super) new)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}
