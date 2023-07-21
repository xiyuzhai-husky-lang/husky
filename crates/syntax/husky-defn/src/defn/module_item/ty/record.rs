use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct RecordTypeNodeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub node_decl: RecordTypeNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeDecl,
}
