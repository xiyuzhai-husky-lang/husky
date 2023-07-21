use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsVariantSynNodeDefn {
    #[id]
    pub node_path: TypeVariantSynNodePath,
    pub node_decl: PropsTypeVariantNodeDecl,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsTypeVariantDecl,
}
