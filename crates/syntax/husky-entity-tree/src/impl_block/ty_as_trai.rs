use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeAsTraitImplBlock {
    #[id]
    pub id: TypeImplId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeAsTraitImplId {
    module_path: ModulePath,
    ty: TypePath,
    trai: TraitPath,
    disambiguator: u8,
}
