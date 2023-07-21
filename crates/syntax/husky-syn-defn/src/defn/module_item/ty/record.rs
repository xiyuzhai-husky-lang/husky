use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct RecordTypeNodeDefn {
    #[id]
    pub node_path: TypeSynNodePath,
    pub node_decl: RecordTypeNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeDecl,
}
