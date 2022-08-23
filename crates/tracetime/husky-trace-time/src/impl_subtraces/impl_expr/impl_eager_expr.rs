use super::*;

impl HuskyTraceTime {
    pub(crate) fn eager_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &EagerExpr,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceId> {
        match expr.variant {
            EagerExprVariant::Variable { .. } => todo!(),
            EagerExprVariant::ThisValue { .. } => todo!(),
            EagerExprVariant::ThisField {
                field_ident,
                field_idx,
                this_ty,
                this_binding,
                field_binding,
            } => todo!(),
            EagerExprVariant::PrimitiveLiteral(_) => todo!(),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.eager_opn_subtraces(parent, expr, history, opn_variant, opds),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
            EagerExprVariant::EntityFeature { .. } => todo!(),
            EagerExprVariant::EntityFp { route } => panic!(),
        }
    }

    pub(crate) fn eager_opn_subtraces(
        &mut self,
        parent: &Trace,
        expr: &EagerExpr,
        history: &Arc<History<'static>>,
        opn_variant: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
    ) -> Vec<TraceId> {
        match opn_variant {
            EagerOpnVariant::Binary { opr, this_ty } => todo!(),
            EagerOpnVariant::Prefix { opr, this_ty } => todo!(),
            EagerOpnVariant::Suffix { this_ty, opr } => todo!(),
            EagerOpnVariant::RoutineCall(route) => {
                let routine_defn = self.runtime().comptime().entity_defn(route.route).unwrap();
                let instruction_sheet = self
                    .runtime()
                    .entity_instruction_sheet(route.route)
                    .unwrap();
                self.routine_call_subtraces(
                    parent,
                    &instruction_sheet,
                    &routine_defn,
                    opds,
                    |this, argument, ident| {
                        (
                            this.new_trace(
                                Some(parent.id()),
                                4,
                                TraceVariant::EagerCallArgument {
                                    argument: argument.clone(),
                                    name: ident,
                                    history: history.clone(),
                                },
                            ),
                            history.register_result(argument),
                        )
                    },
                )
            }
            EagerOpnVariant::TypeCall { ranged_ty, ty_decl } => todo!(),
            EagerOpnVariant::Field {
                this_ty,
                field_ident,
                field_liason,
                field_binding,
                ..
            } => todo!(),
            EagerOpnVariant::MethodCall {
                method_ident,
                this_ty_decl,
                method_route,
                output_binding,
            } => {
                let routine_defn = self
                    .runtime()
                    .comptime()
                    .entity_defn(*method_route)
                    .unwrap();
                let instruction_sheet = self
                    .runtime()
                    .entity_instruction_sheet(*method_route)
                    .unwrap();
                self.routine_call_subtraces(
                    parent,
                    &instruction_sheet,
                    &routine_defn,
                    opds,
                    |this, argument, name| {
                        (
                            this.new_trace(
                                Some(parent.id()),
                                4,
                                TraceVariant::EagerCallArgument {
                                    argument: argument.clone(),
                                    name,
                                    history: history.clone(),
                                },
                            ),
                            history.register_result(argument),
                        )
                    },
                )
            }
            EagerOpnVariant::Index { element_binding } => todo!(),
            EagerOpnVariant::NewVecFromList => todo!(),
            EagerOpnVariant::ValueCall => todo!(),
        }
    }
}
