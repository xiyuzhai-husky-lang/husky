use super::*;

impl Debugtime {
    pub(crate) fn eager_expr_subtraces(
        &mut self,
        parent: &Trace,
        expr: &EagerExpr,
        history: &Arc<History>,
    ) -> Vec<TraceId> {
        match expr.variant {
            EagerExprVariant::Variable { .. } => todo!(),
            EagerExprVariant::ThisValue { .. } => todo!(),
            EagerExprVariant::ThisField { .. } => todo!(),
            EagerExprVariant::PrimitiveLiteral(_) => todo!(),
            EagerExprVariant::Bracketed(_) => todo!(),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.eager_opn_subtraces(parent, history, opn_variant, opds),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
            EagerExprVariant::EntityFeature { .. } => todo!(),
            EagerExprVariant::EntityThickFp { .. } => panic!(),
        }
    }

    pub(crate) fn eager_opn_subtraces(
        &mut self,
        parent: &Trace,
        history: &Arc<History>,
        opn_variant: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
    ) -> Vec<TraceId> {
        todo!()
        // match opn_variant {
        //     EagerOpnVariant::Binary { .. } => todo!(),
        //     EagerOpnVariant::Prefix { .. } => todo!(),
        //     EagerOpnVariant::Suffix { .. } => todo!(),
        //     EagerOpnVariant::RoutineCall(route) => {
        //         let routine_defn = self.runtime().item_defn(route.route).unwrap();
        //         let instruction_sheet = self
        //             .runtime()
        //             .item_instruction_sheet(route.route)
        //             .unwrap();
        //         self.routine_call_subtraces(
        //             parent,
        //             &instruction_sheet,
        //             &routine_defn,
        //             opds,
        //             |this, argument, ident| {
        //                 (
        //                     this.new_trace(
        //                         Some(parent.id()),
        //                         4,
        //                         TraceVariant::EagerCallArgument {
        //                             argument: argument.clone(),
        //                             name: ident,
        //                             history: history.clone(),
        //                         },
        //                     ),
        //                     history.register_result(argument),
        //                 )
        //             },
        //         )
        //     }
        //     EagerOpnVariant::TypeCall { .. } => todo!(),
        //     EagerOpnVariant::Field { .. } => todo!(),
        //     EagerOpnVariant::MethodCall { method_route, .. } => {
        //         let routine_defn = self.runtime().item_defn(*method_route).unwrap();
        //         let instruction_sheet = self
        //             .runtime()
        //             .item_instruction_sheet(*method_route)
        //             .unwrap();
        //         self.routine_call_subtraces(
        //             parent,
        //             &instruction_sheet,
        //             &routine_defn,
        //             opds,
        //             |this, argument, name| {
        //                 (
        //                     this.new_trace(
        //                         Some(parent.id()),
        //                         4,
        //                         TraceVariant::EagerCallArgument {
        //                             argument: argument.clone(),
        //                             name,
        //                             history: history.clone(),
        //                         },
        //                     ),
        //                     history.register_result(argument),
        //                 )
        //             },
        //         )
        //     }
        //     EagerOpnVariant::Index { .. } => todo!(),
        //     EagerOpnVariant::NewVecFromList => todo!(),
        //     EagerOpnVariant::ValueCall => todo!(),
        // }
    }
}
