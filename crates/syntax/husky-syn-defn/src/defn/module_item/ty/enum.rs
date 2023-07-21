use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct EnumTypeNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub node_decl: EnumTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = pub(super) new)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}
