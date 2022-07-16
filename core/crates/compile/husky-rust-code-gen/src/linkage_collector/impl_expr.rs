use super::*;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_from_eager_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable { .. } => (),
            EagerExprVariant::ThisValue { .. } => (),
            EagerExprVariant::ThisField { .. } => (),
            EagerExprVariant::PrimitiveLiteral(_) => (),
            EagerExprVariant::EnumKindLiteral(_) => self.insert(expr.ty()),
            EagerExprVariant::Bracketed(ref expr) => self.collect_from_eager_expr(expr),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => {
                for opd in opds {
                    self.collect_from_eager_expr(opd);
                }
                match opn_variant {
                    EagerOpnVariant::Binary { .. } => (),
                    EagerOpnVariant::Prefix { .. } => (),
                    EagerOpnVariant::Suffix { .. } => (),
                    EagerOpnVariant::RoutineCall(routine) => self.insert(routine.route),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => self.insert(ranged_ty.route),
                    EagerOpnVariant::FieldAccess { .. } => (),
                    EagerOpnVariant::MethodCall { method_route, .. } => match method_route.kind {
                        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                            self.insert(*method_route)
                        }
                        _ => self.insert(*method_route),
                    },
                    EagerOpnVariant::Index { .. } => (),
                }
            }
            EagerExprVariant::Lambda(_, _) => todo!(),
        }
    }
}
