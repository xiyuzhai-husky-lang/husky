pub mod application;

use super::VdTranspileToLean;
use crate::{builder::VdLeanTranspilationBuilder, dictionary::item_path::VdItemPathTranslation};
use lean_mir_expr::expr::{LnMirExprData, LnMirExprIdx};
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::term::literal::{LnLiteral, LnLiteralData};
use visored_mir_expr::expr::{application::VdMirFunc, VdMirExprData, VdMirExprIdx};
use visored_zfc_ty::term::literal::{VdZfcLiteral, VdZfcLiteralData};

impl VdTranspileToLean<LnMirExprIdx> for VdMirExprIdx {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> LnMirExprIdx {
        let data = builder.build_expr(self);
        builder.alloc_expr(data)
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub(crate) fn build_expr(&mut self, expr: VdMirExprIdx) -> LnMirExprData {
        let db = self.db();
        match self.expr_arena()[expr] {
            VdMirExprData::Literal(literal) => LnMirExprData::Literal(to_lean_literal(db, literal)),
            VdMirExprData::ItemPath(item_path) => {
                let Some(translation) = self.dictionary().item_path_translation(item_path) else {
                    todo!()
                };
                match *translation {
                    VdItemPathTranslation::ItemPath(item_path) => {
                        LnMirExprData::ItemPath(item_path)
                    }
                }
            }
            VdMirExprData::Variable(ref vd_hir_variable) => todo!(),
            VdMirExprData::Application {
                function,
                arguments,
            } => self.build_application(expr, function, arguments),
        }
    }
}

#[salsa::tracked]
fn to_lean_literal(db: &salsa::Db, literal: VdZfcLiteral) -> LnLiteral {
    let data = match literal.data(db) {
        VdZfcLiteralData::NaturalNumber(lit) => LnLiteralData::Nat(lit.to_string()),
        VdZfcLiteralData::NegativeInteger(_) => todo!(),
        VdZfcLiteralData::FiniteDecimalRepresentation(_) => {
            todo!()
        }
        VdZfcLiteralData::SpecialConstant(vd_zfc_special_constant) => todo!(),
    };
    LnLiteral::new(data, db)
}
