mod impl_entity_route;
mod impl_expr;
mod impl_lib;
mod impl_mod;
mod impl_routine_defn;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use entity_kind::TyKind;
use entity_syntax::EntityRouteMenu;
use impl_entity_route::*;
use pack_semantics::Package;
use semantics_entity::{EntityDefn, EntityDefnVariant};
use std::sync::Arc;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    result: String,
    package_main: FilePtr,
    entity_route_menu: Arc<EntityRouteMenu>,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustCodeGenQueryGroup, package_main: FilePtr) -> Self {
        Self {
            db,
            package_main,
            result: Default::default(),
            entity_route_menu: db.entity_route_menu(),
        }
    }

    pub(crate) fn package(&self) -> Arc<Package> {
        self.db.package(self.package_main).unwrap()
    }
}
