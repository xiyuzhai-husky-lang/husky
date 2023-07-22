use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct EnumTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_node_decl: EnumTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = pub(super) new)]
pub struct EnumTypeSynDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}
