use crate::EntityDefn;
use crate::*;
use husky_dev_utils::DevSource;
use husky_static_defn::{EntityStaticDefn, MethodStaticDefnKind, StaticTraitImplDefn};
use map_collect::MapCollect;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDefn {
    pub trai: Ty,
    pub member_impls: Vec<Arc<EntityDefn>>,
    pub dev_src: DevSource,
}

impl TraitImplDefn {
    pub fn member_impl(&self, ident: CustomIdentifier) -> &Arc<EntityDefn> {
        self.member_impls
            .iter()
            .find(|member_impl| member_impl.ident.custom() == ident)
            .unwrap()
    }
}
