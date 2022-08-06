mod context;
mod impl_entity_route;
mod impl_expr;
mod impl_init;
mod impl_lib;
mod impl_mod;
mod impl_registration;
mod impl_routine_defn;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;
mod utils;

use crate::*;
use context::*;
use entity_kind::TyKind;
use fold::LocalStack;
use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_package_semantics::Package;
use impl_entity_route::*;
use std::sync::Arc;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    result: String,
    crate_entrance: FilePtr,
    entity_route_uses: LocalStack<EntityRoutePtr>,
    context: RustCodeGenContext,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustCodeGenQueryGroup, module: EntityRoutePtr) -> Self {
        let package_main = db.crate_entrance(db.module_file(module).unwrap()).unwrap();
        let entity_defn = db.entity_defn(module).unwrap();
        let mut symbols = LocalStack::new();
        for entity_defn in entity_defn.subentities.iter() {
            symbols.push(entity_defn.base_route)
        }
        Self {
            db,
            crate_entrance: package_main,
            result: Default::default(),
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
            crate_entrance: package_main,
            result: Default::default(),
            entity_route_uses: symbols,
            context: RustCodeGenContext::Normal,
        }
    }

    pub(crate) fn package(&self) -> Arc<Package> {
        self.db.package(self.crate_entrance).unwrap()
    }

    fn exec_within_context(&mut self, new_context: RustCodeGenContext, f: impl FnOnce(&mut Self)) {
        let old_context = std::mem::replace(&mut self.context, new_context);
        f(self);
        std::mem::replace(&mut self.context, old_context);
    }
}
