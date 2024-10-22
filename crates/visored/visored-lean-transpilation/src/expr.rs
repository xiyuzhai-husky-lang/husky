use super::ToLean;
use crate::builder::VdLeanTranspilationBuilder;
use lean_hir_expr::expr::{LnHirExprData, LnHirExprIdx};
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::term::literal::{LnLiteral, LnLiteralData};
use visored_hir_expr::expr::{application::VdHirApplicationFunction, VdHirExprData, VdHirExprIdx};
use visored_zfs_ty::term::literal::{VdZfsLiteral, VdZfsLiteralData};

impl ToLean for VdHirExprIdx {
    type Target = LnHirExprIdx;

    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> Self::Target {
        let data = builder.build_expr(self);
        builder.alloc_expr(data)
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub(crate) fn build_expr(&mut self, expr: VdHirExprIdx) -> LnHirExprData {
        let db = self.db();
        match self.expr_arena()[expr] {
            VdHirExprData::Literal(literal) => LnHirExprData::Literal(to_lean_literal(db, literal)),
            VdHirExprData::Variable(ref vd_hir_variable) => todo!(),
            VdHirExprData::Application {
                function,
                arguments,
            } => match function {
                VdHirApplicationFunction::IntAdd => {
                    debug_assert_eq!(arguments.len(), 2);
                    let lopd = arguments.start();
                    let ropd = lopd + 1;
                    LnHirExprData::Binary {
                        lopd: lopd.to_lean(self),
                        opr: LnBinaryOpr::Add,
                        ropd: ropd.to_lean(self),
                    }
                }
                VdHirApplicationFunction::TrivialEq => {
                    debug_assert_eq!(arguments.len(), 2);
                    let lopd = arguments.start();
                    let ropd = lopd + 1;
                    LnHirExprData::Binary {
                        lopd: lopd.to_lean(self),
                        opr: LnBinaryOpr::Eq,
                        ropd: ropd.to_lean(self),
                    }
                }
            },
        }
    }
}

#[salsa::tracked]
fn to_lean_literal(db: &salsa::Db, literal: VdZfsLiteral) -> LnLiteral {
    let data = match literal.data(db) {
        VdZfsLiteralData::NaturalNumber(lit) => LnLiteralData::Nat(lit.to_string()),
        VdZfsLiteralData::NegativeInteger(_) => todo!(),
        VdZfsLiteralData::FiniteDecimalRepresentation(_) => {
            todo!()
        }
        VdZfsLiteralData::SpecialConstant(vd_zfs_special_constant) => todo!(),
    };
    LnLiteral::new(data, db)
}
