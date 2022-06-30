mod impl_expr;
mod impl_lib;
mod impl_mod;
mod impl_routine;
mod impl_scope;
mod impl_stmt;
mod impl_ty_defn;
mod impl_write;

use crate::*;
use entity_kind::TyKind;
use pack_semantics::Package;
use semantics_entity::{EntityDefn, EntityDefnVariant};
use std::sync::Arc;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    indent: fold::Indent,
    result: String,
    package_main: FilePtr,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(db: &'a dyn RustCodeGenQueryGroup, package_main: FilePtr) -> Self {
        Self {
            db,
            package_main,
            indent: 0,
            result: Default::default(),
        }
    }

    pub(crate) fn package(&self) -> Arc<Package> {
        self.db.package(self.package_main).unwrap()
    }
}
