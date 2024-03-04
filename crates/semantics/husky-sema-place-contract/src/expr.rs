use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
use husky_fly_term::{signature::FlyFieldSignature, ExpectationOutcome, FlyCoersion};
#[allow(unused_imports)]
use husky_sema_expr::emit_note_on_sema_expr_codespan;
use husky_sema_expr::{SemaExprData, SemaExprIdx, SemaRitchieParameterArgumentMatch};
use husky_sema_opr::{binary::SemaBinaryOpr, prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::SynExprRootKind;
use husky_term_prelude::Contract;

impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn infer_all_exprs(&mut self) {
        for (expr, root_kind) in self.sema_expr_region_data().sema_expr_roots() {
            if let Some(contract) = root_contract(root_kind) {
                let site = SemaPlaceContractSite::default();
                self.infer_expr(expr, contract, site)
            }
        }
    }

    pub(crate) fn infer_expr(
        &mut self,
        expr: SemaExprIdx,
        outer_contract: Contract,
        outer_site: SemaPlaceContractSite,
    ) {
        let (contract, mut site) = if let ExpectationOutcome::Coersion(outcome) = expr
            .expectation_outcome(self.sema_expr_region_data())
            .expect("no semantic expr error at this stage")
        {
            match outcome.coersion() {
                FlyCoersion::Trivial(_) => (outer_contract, outer_site),
                FlyCoersion::Never | FlyCoersion::PlaceToLeash | FlyCoersion::Deref(_) => {
                    (Contract::Pure, Default::default())
                }
                FlyCoersion::WrapInSome => (Contract::Move, Default::default()),
            }
        } else {
            (outer_contract, outer_site)
        };
        if let Some(place) = self.place(expr) {
            site.set(place, contract)
        }
        self.infer_subexprs(expr, contract, &site);
        self.set_expr_site(expr, site)
    }

    fn infer_subexprs(
        &mut self,
        parent_expr: SemaExprIdx,
        contract: Contract,
        site: &SemaPlaceContractSite,
    ) {
        match *parent_expr.data(self.sema_expr_region_data().sema_expr_arena()) {
            SemaExprData::Literal(_, _)
            | SemaExprData::Unit { .. }
            | SemaExprData::PrincipalEntityPath { .. }
            | SemaExprData::AssocItem { .. }
            | SemaExprData::InheritedSynSymbol { .. }
            | SemaExprData::CurrentSynSymbol { .. }
            | SemaExprData::FrameVarDecl { .. }
            | SemaExprData::SelfType(_)
            | SemaExprData::SelfValue(_)
            | SemaExprData::FunctionApplication { .. } => (),
            SemaExprData::Binary {
                lopd, opr, ropd, ..
            } => {
                // todo: coersion?
                let (lopd_contract, ropd_contract) = match opr {
                    SemaBinaryOpr::Assign => (Contract::BorrowMut, Contract::Move),
                    SemaBinaryOpr::AssignClosed(_) | SemaBinaryOpr::AssignShift(_) => {
                        (Contract::BorrowMut, Contract::Pure)
                    }
                    _ => (Contract::Pure, Contract::Pure),
                };
                self.infer_expr(lopd, lopd_contract, Default::default());
                self.infer_expr(ropd, ropd_contract, Default::default());
            }
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => todo!(),
            SemaExprData::Prefix { opr, opd, .. } => {
                let contract = match opr {
                    SemaPrefixOpr::Minus | SemaPrefixOpr::Not | SemaPrefixOpr::BitNot => {
                        Contract::Pure
                    }
                    SemaPrefixOpr::LeashType
                    | SemaPrefixOpr::RefType
                    | SemaPrefixOpr::OptionType => return,
                };
                self.infer_expr(opd, contract, site.clone())
            }
            SemaExprData::Suffix { opd, opr, .. } => {
                let contract = match opr {
                    SemaSuffixOpr::Incr | SemaSuffixOpr::Decr => Contract::BorrowMut,
                    SemaSuffixOpr::ComposeWithOption | SemaSuffixOpr::ComposeWithNot => return,
                };
                self.infer_expr(opd, contract, site.clone())
            }
            SemaExprData::Unveil {
                opd_sema_expr_idx, ..
            } => self.infer_expr(opd_sema_expr_idx, Contract::Move, site.clone()),
            SemaExprData::Unwrap {
                opd_sema_expr_idx, ..
            } => self.infer_expr(opd_sema_expr_idx, Contract::At, site.clone()),
            SemaExprData::FunctionRitchieCall {
                function_sema_expr_idx,
                ritchie_ty_kind,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                self.infer_expr(
                    function_sema_expr_idx,
                    ritchie_ty_kind.function_contract(),
                    Default::default(),
                );
                self.infer_ritchie_parameter_argument_matches(ritchie_parameter_argument_matches);
            }
            SemaExprData::Ritchie { .. } => (),
            SemaExprData::Field {
                owner,
                owner_ty,
                dot_regional_token_idx,
                ident_token,
                ref dispatch,
            } => match dispatch.signature() {
                FlyFieldSignature::PropsStruct { .. } => {
                    self.infer_expr(owner, contract, site.clone())
                }
                FlyFieldSignature::Memoized { .. } => {
                    self.infer_expr(owner, Contract::Leash, Default::default())
                }
            },
            SemaExprData::MethodApplication { .. } => (),
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                self_contract,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                self.infer_expr(self_argument_sema_expr_idx, self_contract, site.clone());
                self.infer_ritchie_parameter_argument_matches(ritchie_parameter_argument_matches)
            }
            SemaExprData::MethodGnCall {
                self_argument_sema_expr_idx,
                dot_regional_token_idx,
                ident_token,
                ref method_dynamic_dispatch,
                ref template_arguments,
                lpar_regional_token_idx,
                ref ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemaExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemaExprData::Delimitered { item, .. } => self.infer_expr(item, contract, site.clone()),
            SemaExprData::NewTuple { ref items, .. } => {
                for item in items {
                    self.infer_expr(item.sema_expr_idx, Contract::Move, Default::default())
                }
            }
            SemaExprData::Index {
                owner,
                ref index_sema_list_items,
                ..
            } => {
                self.infer_expr(owner, contract, site.clone());
                for item in index_sema_list_items {
                    self.infer_expr(item.sema_expr_idx, contract, Default::default());
                }
            }
            SemaExprData::CompositionWithList { .. } => (),
            SemaExprData::NewList { ref items, .. } => {
                for item in items {
                    self.infer_expr(item.sema_expr_idx, Contract::Move, Default::default())
                }
            }
            SemaExprData::BoxColonList { .. } => (),
            SemaExprData::VecFunctor { .. } => (),
            SemaExprData::ArrayFunctor { .. } => (),
            SemaExprData::Block { stmts } => self.infer_stmts(stmts, contract, site.clone()),
            SemaExprData::EmptyHtmlTag { ref arguments, .. } => {
                for arg in arguments {
                    self.infer_expr(arg.expr(), Contract::Pure, Default::default())
                }
            }
            SemaExprData::Closure { body, .. } => {
                self.infer_expr(body, Contract::Move, Default::default())
            }
            // todo: macro arguments
            SemaExprData::Sorry { .. } => (),
            // todo: macro arguments
            SemaExprData::Todo { .. } => (),
            // todo: macro arguments
            SemaExprData::Unreachable { .. } => (),
            SemaExprData::NestedBlock { stmts, .. } => {
                self.infer_stmts(stmts, contract, site.clone())
            }
        }
    }

    fn infer_ritchie_parameter_argument_matches(
        &mut self,
        ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    ) {
        for m in ritchie_parameter_argument_matches {
            match m {
                SemaRitchieParameterArgumentMatch::Simple(param, arg) => {
                    self.infer_expr(arg.argument_expr_idx, param.contract, Default::default())
                }
                SemaRitchieParameterArgumentMatch::Variadic(param, args) => {
                    for arg in args {
                        self.infer_expr(
                            arg.argument_expr_idx(),
                            param.contract(),
                            Default::default(),
                        )
                    }
                }
                SemaRitchieParameterArgumentMatch::Keyed(param, arg) => {
                    if let Some(arg) = arg {
                        self.infer_expr(
                            arg.argument_expr_idx(),
                            param.contract(),
                            Default::default(),
                        )
                    }
                }
            }
        }
    }
}

fn root_contract(root_kind: SynExprRootKind) -> Option<Contract> {
    match root_kind {
        SynExprRootKind::SelfType => None,
        SynExprRootKind::Trait => None,
        SynExprRootKind::ReturnType => None,
        SynExprRootKind::PropsStructFieldType { .. } => None,
        SynExprRootKind::TupleStructFieldType => None,
        SynExprRootKind::BlockExpr => Some(Contract::Move),
        SynExprRootKind::ReturnExpr => Some(Contract::Move),
        SynExprRootKind::Condition => todo!(),
        SynExprRootKind::ParenateParameterDefaultValue { .. } => Some(Contract::Move),
        SynExprRootKind::FieldBindInitialValue { .. } => Some(Contract::Move),
        SynExprRootKind::ConstantImplicitParameterType => None,
        SynExprRootKind::ExplicitParameterType => None,
        SynExprRootKind::HtmlArgumentExpr => todo!(),
        SynExprRootKind::LetStmtType => None,
        SynExprRootKind::LetStmtInitialValue => todo!(),
        SynExprRootKind::Snippet => todo!(),
        SynExprRootKind::ValExpr => todo!(),
        SynExprRootKind::EvalExpr => todo!(),
        SynExprRootKind::AssocTypeTerm => None,
        SynExprRootKind::TypeAliasTypeTerm => None,
    }
}
