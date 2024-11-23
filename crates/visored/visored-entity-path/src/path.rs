pub mod category;
pub mod function;
pub mod set;
pub mod trai;
pub mod trai_item;

use self::{category::*, function::*, set::*, trai::*, trai_item::*};
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdItemPath {
    Category(VdCategoryPath),
    Set(VdSetPath),
    Function(VdFunctionPath),
    Trait(VdTraitPath),
    TraitItem(VdTraitItemPath),
}

impl VdItemPath {
    // # categories
    pub const SET: Self = VdItemPath::Category(VdCategoryPath::SET);
    pub const PROPOSITION: Self = VdItemPath::Category(VdCategoryPath::PROPOSITION);
    // # sets
    pub const NAT: Self = VdItemPath::Set(VdSetPath::NATURAL_NUMBER);
    pub const RAT: Self = VdItemPath::Set(VdSetPath::RATIONAL_NUMBER);
    pub const INT: Self = VdItemPath::Set(VdSetPath::INTEGER);
    pub const REAL: Self = VdItemPath::Set(VdSetPath::REAL_NUMBER);
    pub const COMPLEX: Self = VdItemPath::Set(VdSetPath::COMPLEX_NUMBER);
    // # functions
    pub const SIN: Self = VdItemPath::Function(VdFunctionPath::SIN);
    pub const COS: Self = VdItemPath::Function(VdFunctionPath::COS);
    // # trait items
    pub const EQ: Self = VdItemPath::TraitItem(VdTraitItemPath::EQ);
    pub const NE: Self = VdItemPath::TraitItem(VdTraitItemPath::NE);
    pub const LT: Self = VdItemPath::TraitItem(VdTraitItemPath::LT);
    pub const GT: Self = VdItemPath::TraitItem(VdTraitItemPath::GT);
    pub const LE: Self = VdItemPath::TraitItem(VdTraitItemPath::LE);
    pub const GE: Self = VdItemPath::TraitItem(VdTraitItemPath::GE);
    pub const NAT_ADD: Self = VdItemPath::TraitItem(VdTraitItemPath::NAT_ADD);
    pub const NAT_MUL: Self = VdItemPath::TraitItem(VdTraitItemPath::NAT_MUL);
    pub const RING_ADD: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_ADD);
    pub const RING_MUL: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_MUL);
    pub const RING_POWER: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_POWER);
    pub const RING_POS: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_POS);
    pub const RING_NEG: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_NEG);
    pub const RING_SUB: Self = VdItemPath::TraitItem(VdTraitItemPath::RING_SUB);
    pub const FIELD_DIV: Self = VdItemPath::TraitItem(VdTraitItemPath::FIELD_DIV);
    pub const REAL_SQRT: Self = VdItemPath::TraitItem(VdTraitItemPath::REAL_SQRT);
}

impl VdItemPath {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr) -> Self {
        let LpCsvExprData::Ident(ref ident) = expr.data else {
            todo!("expected identifier but got {:#?}", expr.data)
        };
        match ident as &str {
            "eq" => VdItemPath::EQ,
            "ne" => VdItemPath::NE,
            "lt" => VdItemPath::LT,
            "gt" => VdItemPath::GT,
            "le" => VdItemPath::LE,
            "ge" => VdItemPath::GE,
            "ring_pos" => VdItemPath::RING_POS,
            "ring_neg" => VdItemPath::RING_NEG,
            "ring_sub" => VdItemPath::RING_SUB,
            "nat_add" => VdItemPath::NAT_ADD,
            "nat_mul" => VdItemPath::NAT_MUL,
            "ring_add" => VdItemPath::RING_ADD,
            "ring_mul" => VdItemPath::RING_MUL,
            "ring_power" => VdItemPath::RING_POWER,
            "field_div" => VdItemPath::FIELD_DIV,
            "real_sqrt" => VdItemPath::REAL_SQRT,
            s => todo!("s = {s:?} not handled"),
        }
    }
}

impl salsa::DebugWithDb for VdItemPath {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;

        self.display_fmt_with_db(f, db)
    }
}

impl salsa::DisplayWithDb for VdItemPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        match self {
            VdItemPath::Category(category_path) => category_path.display_fmt_with_db(f, db),
            VdItemPath::Set(set_path) => set_path.display_fmt_with_db(f, db),
            VdItemPath::Function(function_path) => function_path.display_fmt_with_db(f, db),
            VdItemPath::Trait(trait_path) => trait_path.display_fmt_with_db(f, db),
            VdItemPath::TraitItem(trait_item_path) => trait_item_path.display_fmt_with_db(f, db),
        }
    }
}
