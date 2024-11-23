pub mod table;

use crate::{
    menu::{vd_ty_menu, VdTypeMenu},
    term::{VdTerm, VdTermData, VdTermId},
};
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};
use smallvec::{smallvec, SmallVec};
use visored_coword::namae::VdNamae;
use visored_entity_path::path::VdItemPath;

#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdType(VdTerm);

impl salsa::DebugWithDb for VdType {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;

        self.display_fmt_with_db(f, db)
    }
}

impl salsa::DisplayWithDb for VdType {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        self.0.display_fmt_with_db(f, db)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdTypeData {
    ItemPath(VdItemPath), // TODO: do we need a path here?
}

impl VdType {
    pub fn new_item_path(item_path: VdItemPath, db: &::salsa::Db) -> Self {
        // TODO: check this is actually a type?
        VdType(VdTerm::new_item_path(item_path, db))
    }
}

impl VdType {
    pub fn is_function_like(self, db: &::salsa::Db) -> bool {
        is_vd_ty_function_like(db, **self)
    }
}

#[salsa::tracked]
fn is_vd_ty_function_like(db: &::salsa::Db, ty: VdTermId) -> bool {
    // TODO: ad hoc implementation
    match *ty.data(db) {
        VdTermData::ItemPath(_) => false,
        VdTermData::Literal(_) => todo!(),
        VdTermData::ForAll(_) => todo!(),
        VdTermData::Exists(_) => todo!(),
        VdTermData::Limit(_) => todo!(),
        VdTermData::Eval(_) => todo!(),
        VdTermData::SymbolicVariable(_) => todo!(),
        VdTermData::AbstractVariable(_) => todo!(),
        VdTermData::StackVariable(_) => todo!(),
        VdTermData::Application(_) => todo!(),
        VdTermData::Abstraction(_) => todo!(),
    }
}

impl VdType {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &::salsa::Db) -> Self {
        match expr.data {
            LpCsvExprData::Literal(ref literal) => todo!(),
            LpCsvExprData::Application(ref app) => todo!(),
            LpCsvExprData::List(ref vec) => todo!(),
            LpCsvExprData::Ident(ref ident) => Self::from_lp_csv_ident(ident, db),
            LpCsvExprData::Parenthesized(ref lp_csv_expr) => todo!(),
        }
    }

    pub fn from_lp_csv_ident(ident: &str, db: &::salsa::Db) -> Self {
        let VdTypeMenu {
            nat,
            int,
            rat,
            real,
            complex,
            set,
            prop,
        } = *vd_ty_menu(db);
        match ident as &str {
            "true" => todo!(),
            "false" => todo!(),
            "nat" => nat,
            "int" => int,
            "rat" => rat,
            "real" => real,
            "complex" => complex,
            "set" => set,
            "prop" => prop,
            s => todo!("s = {s:?} not handled"),
        }
    }
}
