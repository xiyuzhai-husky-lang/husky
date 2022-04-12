use atom::symbol_proxy::Symbol;
use fold::LocalStack;
use map_collect::MapCollect;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDecl {
    pub trait_decl: Arc<TraitDecl>,
    pub this_ty: EntityRoutePtr,
    pub members: Vec<TraitMemberImplDecl>,
}

impl TraitImplDecl {
    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        static_trait_impl: &StaticTraitImplDecl,
        this_ty: EntityRoutePtr,
        symbols: &LocalStack<Symbol>,
    ) -> Arc<Self> {
        let route = db
            .parse_entity(static_trait_impl.route, Some(this_ty), symbols)
            .unwrap();
        let trait_decl = db.trait_decl(route).unwrap();
        Self::from_trait_decl(db, trait_decl, this_ty)
    }

    pub(crate) fn from_trait_decl(
        db: &dyn DeclQueryGroup,
        trait_decl: Arc<TraitDecl>,
        this_ty: EntityRoutePtr,
    ) -> Arc<Self> {
        let memb_impls = trait_decl
            .members
            .map(|member| member.implement(db, this_ty));
        Arc::new(Self {
            trait_decl,
            this_ty,
            members: memb_impls,
        })
    }

    pub(crate) fn clone_trait_impl(db: &dyn DeclQueryGroup, this_ty: EntityRoutePtr) -> Arc<Self> {
        Self::from_trait_decl(db, db.trait_decl_menu().clone_trait.clone(), this_ty)
    }

    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            trait_decl: self.trait_decl.instantiate(instantiator),
            this_ty: instantiator
                .instantiate_entity_route(self.this_ty)
                .as_scope(),
            members: self.members.map(|member| member.instantiate(instantiator)),
        })
    }
}
