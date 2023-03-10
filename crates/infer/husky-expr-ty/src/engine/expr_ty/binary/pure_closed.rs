use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: ExprIdx,
        ropd: ExprIdx,
        opr: BinaryClosedOpr,
        menu: &TermMenu,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        // todo: don't use resolved
        let Some(lopd_ty) = self.infer_new_expr_ty(
            lopd, ExpectAnyOriginal, local_term_region
        ) else {
            return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
        };
        let lopd_ty_unravelled =
            lopd_ty.unravel_borrow(self.db, local_term_region.unresolved_terms());
        match lopd_ty_unravelled.pattern(self.db, local_term_region.unresolved_terms()) {
            LocalTermPattern::TypeOntology {
                refined_path: Right(PreludeTypePath::Num(_)),
                ..
            }
            | LocalTermPattern::ImplicitSymbol(
                ImplicitSymbolKind::UnspecifiedIntegerType
                | ImplicitSymbolKind::UnspecifiedFloatType,
                _,
            ) => {
                self.infer_new_expr_ty(
                    ropd,
                    ExpectImplicitlyConvertible {
                        destination: lopd_ty_unravelled,
                    },
                    local_term_region,
                );
                Ok(lopd_ty_unravelled)
            }
            LocalTermPattern::TypeOntology { .. }
            | LocalTermPattern::ImplicitSymbol(_, _)
            | LocalTermPattern::Literal(_)
            | LocalTermPattern::Curry {}
            | LocalTermPattern::Category(_) => todo!(),
        }
    }
}
// let lopd_ty: Term = todo!();
// //  self.db.intrinsic_ty(lopd_ty).reduced_term();
// let ropd_ty: Term = todo!();
// //  self.db.intrinsic_ty(ropd_ty).reduced_term();
// match opr {
//     BinaryPureClosedOpr::Add => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::BitAnd => match lopd_ty {
//         lopd_ty if lopd_ty == menu.r32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r32() => Ok(menu.r32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.r64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r64() => Ok(menu.r64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::BitOr => match lopd_ty {
//         lopd_ty if lopd_ty == menu.r32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r32() => Ok(menu.r32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.r64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r64() => Ok(menu.r64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::BitXor => match lopd_ty {
//         lopd_ty if lopd_ty == menu.r32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r32() => Ok(menu.r32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.r64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.r64() => Ok(menu.r64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::Div => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::Mul => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.f32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.f32() => Ok(menu.f32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.f64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.f64() => Ok(menu.f64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::RemEuclid => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::Power => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::Shl => todo!(),
//     BinaryPureClosedOpr::Shr => match lopd_ty {
//         lopd_ty if lopd_ty == menu.r32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.r32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.r64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.r64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
//     BinaryPureClosedOpr::Sub => match lopd_ty {
//         lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
//             _ => Err(todo!()),
//         },
//         lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
//             ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
//             _ => Err(todo!()),
//         },
//         _ => Err(todo!()),
//     },
// }
