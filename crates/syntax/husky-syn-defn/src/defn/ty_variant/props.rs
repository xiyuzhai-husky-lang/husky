use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct PropsVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: PropsTypeVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsTypeVariantDecl,
}
