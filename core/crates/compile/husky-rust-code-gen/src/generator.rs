mod context;
mod impl_entity_route;
mod impl_expr;
mod impl_lib;
mod impl_mod;
mod impl_routine_defn;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;
mod utils;

use crate::*;
use context::*;
use entity_kind::TyKind;
use entity_syntax::EntityRouteMenu;
use fold::LocalStack;
use impl_entity_route::*;
use pack_semantics::Package;
use semantics_entity::{EntityDefn, EntityDefnVariant};
use std::sync::Arc;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    result: String,
    package_main: FilePtr,
    entity_route_menu: Arc<EntityRouteMenu>,
    entity_route_uses: LocalStack<EntityRoutePtr>,
    context: RustCodeGenContext,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustCodeGenQueryGroup, module: EntityRoutePtr) -> Self {
        let package_main = db.main_file(db.module_file(module).unwrap()).unwrap();
        let entity_defn = db.entity_defn(module).unwrap();
        let mut symbols = LocalStack::new();
        for entity_defn in entity_defn.subentities.iter() {
            symbols.push(entity_defn.base_route)
        }
        Self {
            db,
            package_main,
            result: Default::default(),
            entity_route_menu: db.entity_route_menu(),
            entity_route_uses: symbols,
            context: RustCodeGenContext::Normal,
        }
    }
    pub(crate) fn new_lib(db: &'a dyn RustCodeGenQueryGroup, package_main: FilePtr) -> Self {
        let mut symbols = LocalStack::new();
        let package = db.package(package_main).unwrap();
        for entity_defn in package.subentities.iter() {
            symbols.push(entity_defn.base_route)
        }
        Self {
            db,
            package_main,
            result: Default::default(),
            entity_route_menu: db.entity_route_menu(),
            entity_route_uses: symbols,
            context: RustCodeGenContext::Normal,
        }
    }

    pub(crate) fn package(&self) -> Arc<Package> {
        self.db.package(self.package_main).unwrap()
    }

    fn exec_within_context(&mut self, new_context: RustCodeGenContext, f: impl FnOnce(&mut Self)) {
        let old_context = std::mem::replace(&mut self.context, new_context);
        f(self);
        std::mem::replace(&mut self.context, old_context);
    }
}
