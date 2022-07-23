use super::*;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use husky_lazy_semantics::{LazyExpr, LazyExprVariant, LazyOpnKind};

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
                    EagerOpnVariant::Field { .. } => (),
                    EagerOpnVariant::MethodCall { method_route, .. } => match method_route.kind {
                        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                            self.insert(*method_route)
                        }
                        _ => self.insert(*method_route),
                    },
                    EagerOpnVariant::Index { .. } => (),
                    EagerOpnVariant::NewVecFromList => self.insert(expr.ty()),
                    EagerOpnVariant::ValueCall => todo!(),
                }
            }
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EntityFeature { route } => self.insert(route),
            EagerExprVariant::EntityFp { route } => todo!(),
        }
    }

    pub(crate) fn collect_from_lazy_expr(&mut self, expr: &LazyExpr) {
        match expr.variant {
            LazyExprVariant::Variable { varname, binding } => (),
            LazyExprVariant::PrimitiveLiteral(_) => (),
            LazyExprVariant::EnumLiteral { entity_route } => self.insert(entity_route.parent()),
            LazyExprVariant::Bracketed(ref bracketed_expr) => {
                self.collect_from_lazy_expr(bracketed_expr)
            }
            LazyExprVariant::Opn { opn_kind, ref opds } => {
                for opd in opds {
                    self.collect_from_lazy_expr(opd);
                }
                match opn_kind {
                    LazyOpnKind::Binary { opr, this } => (),
                    LazyOpnKind::Prefix(_) => todo!(),
                    LazyOpnKind::FunctionModelCall(ranged_route) => self.insert(ranged_route.route),
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => {
                        self.insert(ranged_route.route)
                    }
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::NewVecFromList => self.insert(expr.ty()),
                    LazyOpnKind::RecordCall(ranged_route) => self.insert(ranged_route.route),
                    LazyOpnKind::Field { .. } => (),
                    LazyOpnKind::MethodCall { method_route, .. } => self.insert(method_route),
                    LazyOpnKind::Index { .. } => todo!(),
                }
            }
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::ThisValue { binding } => todo!(),
            LazyExprVariant::ThisField {
                field_ident,
                this_ty,
                this_binding,
                field_binding,
            } => todo!(),
            LazyExprVariant::EntityFeature { entity_route } => self.insert(entity_route),
        }
    }
}
