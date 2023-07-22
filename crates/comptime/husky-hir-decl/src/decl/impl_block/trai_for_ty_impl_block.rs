use super::*;
use smallvec::SmallVec;

#[salsa::tracked(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct TraitForTypeImplBlockHirDecl {
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub trai: EtherealTerm,
    pub self_ty: EtherealSelfType,
    // todo: where clause
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSelfType {
    PathLeading(EtherealTerm),
    DeriveAny(EtherealTermSymbol),
}

impl HasHirDecl for TraitForTypeImplBlockPath {
    type HirDecl = TraitForTypeImplBlockHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        trai_for_ty_impl_block_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_impl_block_hir_decl(
    db: &dyn HirDeclDb,
    path: TraitForTypeImplBlockPath,
) -> TraitForTypeImplBlockHirDecl {
    todo!()
    // TraitForTypeImplBlockHirDecl::from_declarative(
    //     db,
    //     path,
    //     path.declarative_signature_template(db)?,
    // )
}
