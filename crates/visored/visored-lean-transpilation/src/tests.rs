use visored_hir_expr::{builder::VdHirExprBuilder, ToVdHir};
use visored_opr::opr::binary::VdBinaryOpr;
use visored_sem_expr::{
    expr::{binary::VdSemBinaryDispatch, literal::VdSemLiteralDispatch, VdSemExprData},
    test_helpers::builder::VdSemExprTestBuilder,
};
use visored_zfs_ty::menu::vd_zfs_ty_menu;

use crate::{builder::VdLeanTranspilationBuilder, ToLean};

#[salsa::db(
    husky_coword::jar::CowordJar,
    lean_term::jar::LnTermJar,
    lean_sem_expr::jar::LnSemExprJar,
    lean_hir_expr::jar::LnHirExprJar,
    visored_zfs_ty::jar::VdZfsTypeJar,
    visored_opr::jar::VdOprJar,
    visored_sem_expr::jar::VdSemExprJar,
    visored_hir_expr::jar::VdHirExprJar,
    crate::Jar
)]
pub(crate) struct DB {}

#[test]
fn to_lean_works() {
    let db = &DB::default();
    let menu = vd_zfs_ty_menu(db);
    let mut builder = VdSemExprTestBuilder::new(db);
    let one = builder.new_expr_checked(
        VdSemExprData::Literal {
            literal: menu.one_literal(),
            dispatch: VdSemLiteralDispatch::Int,
        },
        "1",
    );
    let one_add_one = builder.new_expr_checked(
        VdSemExprData::Binary {
            lopd: one,
            opr: VdBinaryOpr::Add,
            ropd: one,
            dispatch: VdSemBinaryDispatch::IntAdd,
        },
        "1+1",
    );
    let vd_sem_expr_region_data = &builder.finish();
    let mut builder = VdHirExprBuilder::new(db, vd_sem_expr_region_data);
    let one_add_one = one_add_one.to_hir(&mut builder);
    let vd_hir_expr_region_data = &builder.finish();
    let mut builder = VdLeanTranspilationBuilder::new(db, vd_hir_expr_region_data);
    let one_add_one = one_add_one.to_lean(&mut builder);
}
