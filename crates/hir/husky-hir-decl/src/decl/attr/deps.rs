use super::*;
use husky_eth_term::term::EthTerm;
use husky_syn_decl::decl::attr::deps::DepsAttrSynDecl;
use husky_term_prelude::ItemPathTerm;

#[salsa::interned]
pub struct DepsAttrHirDecl {
    pub path: AttrItemPath,
    #[return_ref]
    pub deps: Vec<ItemPathTerm>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl DepsAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: DepsAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let deps = syn_decl
            .deps(db)
            .iter()
            .map(|&dep| match builder.eth_term(dep) {
                Some(dep_term) => match dep_term {
                    EthTerm::Literal(_) => todo!(),
                    EthTerm::SymbolicVariable(_) => todo!(),
                    EthTerm::LambdaVariable(_) => todo!(),
                    EthTerm::ItemPath(path) => path,
                    EthTerm::Sort(_) => todo!(),
                    EthTerm::Universe(_) => todo!(),
                    EthTerm::Curry(_) => todo!(),
                    EthTerm::Ritchie(_) => todo!(),
                    EthTerm::Abstraction(_) => todo!(),
                    EthTerm::Application(_) => todo!(),
                    EthTerm::TypeAsTraitItem(_) => todo!(),
                    EthTerm::TraitConstraint(_) => todo!(),
                },
                None => todo!(),
            })
            .collect();
        DepsAttrHirDecl::new(db, path, deps, builder.finish().eager())
    }
}
