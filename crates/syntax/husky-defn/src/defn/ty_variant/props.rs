use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsVariantNodeDefn {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub node_decl: PropsVariantNodeDecl,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct PropsVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: PropsVariantDecl,
}
