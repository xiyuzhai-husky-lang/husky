use crate::{engine::PlaceContractEngine, site::SemPlaceContractSite};
use husky_fly_term::{signature::FlyFieldSignature, ExpectationOutcome, FlyCoercion};
#[allow(unused_imports)]
use husky_sem_expr::emit_note_on_sem_expr_codespan;
use husky_sem_expr::{SemExprData, SemExprIdx, SemaRitchieArgument};
use husky_sem_opr::{binary::SemaBinaryOpr, prefix::SemaPrefixOpr, suffix::SemaSuffixOpr};
use husky_syn_expr::context::SynExprRootKind;
use husky_term_prelude::Contract;

impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn infer_all_exprs(&mut self) {
        for (expr, root_kind) in self.sem_expr_region_data().sem_expr_roots() {
            if let Some(contract) = root_contract(root_kind) {
                let site = SemPlaceContractSite::default();
                self.infer_expr(expr, contract, site)
            }
        }
    }

    pub(crate) fn infer_expr(
        &mut self,
        expr: SemExprIdx,
        outer_contract: Contract,
        outer_site: SemPlaceContractSite,
    ) {
        let (contract, mut site) = if let ExpectationOutcome::Coercion(outcome) = expr
            .expectation_outcome(self.sem_expr_region_data())
            .expect("no semantic expr error at this stage")
        {
            match outcome.coercion() {
                FlyCoercion::Trivial(_) => (outer_contract, outer_site),
                FlyCoercion::Never | FlyCoercion::PlaceToLeash | FlyCoercion::Deref(_) => {
                    (Contract::Pure, Default::default())
                }
                FlyCoercion::WrapInSome => (Contract::Move, Default::default()),
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
        parent_expr: SemExprIdx,
        contract: Contract,
        site: &SemPlaceContractSite,
    ) {
        match *parent_expr.data(self.sem_expr_region_data().sem_expr_arena()) {
            SemExprData::Literal(_, _)
            | SemExprData::Unit { .. }
            | SemExprData::PrincipalEntityPath { .. }
            | SemExprData::MajorItemPathAssocItem { .. }
            | SemExprData::TypeAsTraitItem { .. }
            | SemExprData::AssocItem { .. }
            | SemExprData::InheritedSynSymbol { .. }
            | SemExprData::CurrentSynSymbol { .. }
            | SemExprData::FrameVarDecl { .. }
            | SemExprData::SelfType(_)
            | SemExprData::SelfValue(_)
            | SemExprData::FunctionApplication { .. } => (),
            SemExprData::Binary {
                lopd, opr, ropd, ..
            } => {
                // todo: coercion?
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
            SemExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => todo!(),
            SemExprData::Prefix { opr, opd, .. } => {
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
            SemExprData::Suffix { opd, opr, .. } => {
                let contract = match opr {
                    SemaSuffixOpr::Incr | SemaSuffixOpr::Decr => Contract::BorrowMut,
                    SemaSuffixOpr::ComposeWithOption | SemaSuffixOpr::ComposeWithNot => return,
                };
                self.infer_expr(opd, contract, site.clone())
            }
            SemExprData::Unveil {
                opd: opd_sem_expr_idx,
                ..
            } => self.infer_expr(opd_sem_expr_idx, Contract::Move, site.clone()),
            SemExprData::Unwrap {
                opd: opd_sem_expr_idx,
                ..
            } => self.infer_expr(opd_sem_expr_idx, Contract::At, site.clone()),
            SemExprData::FunctionRitchieCall {
                function: function_sem_expr_idx,
                ritchie_ty_kind,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                self.infer_expr(
                    function_sem_expr_idx,
                    ritchie_ty_kind.function_contract(),
                    Default::default(),
                );
                self.infer_ritchie_parameter_argument_matches(ritchie_parameter_argument_matches);
            }
            SemExprData::Ritchie { .. } => (),
            SemExprData::Field {
                self_argument: owner,
                self_ty: owner_ty,
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
            SemExprData::MethodApplication { .. } => (),
            SemExprData::MethodFnCall {
                self_argument: self_argument_sem_expr_idx,
                self_contract,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                self.infer_expr(self_argument_sem_expr_idx, self_contract, site.clone());
                self.infer_ritchie_parameter_argument_matches(ritchie_parameter_argument_matches)
            }
            SemExprData::MethodGnCall {
                self_argument: self_argument_sem_expr_idx,
                dot_regional_token_idx,
                ident_token,
                ref method_dynamic_dispatch,
                ref template_arguments,
                lpar_regional_token_idx,
                ref ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemExprData::Delimitered { item, .. } => self.infer_expr(item, contract, site.clone()),
            SemExprData::NewTuple { ref items, .. } => {
                for item in items {
                    self.infer_expr(item.sem_expr_idx, Contract::Move, Default::default())
                }
            }
            SemExprData::Index {
                owner,
                ref index_sem_list_items,
                ..
            } => {
                self.infer_expr(owner, contract, site.clone());
                for item in index_sem_list_items {
                    self.infer_expr(item.sem_expr_idx, contract, Default::default());
                }
            }
            SemExprData::CompositionWithList { .. } => (),
            SemExprData::NewList { ref items, .. } => {
                for item in items {
                    self.infer_expr(item.sem_expr_idx, Contract::Move, Default::default())
                }
            }
            SemExprData::BoxColonList { .. } => (),
            SemExprData::VecFunctor { .. } => (),
            SemExprData::ArrayFunctor { .. } => (),
            SemExprData::Block { stmts } => self.infer_stmts(stmts, contract, site.clone()),
            SemExprData::EmptyHtmlTag { ref arguments, .. } => {
                for arg in arguments {
                    self.infer_expr(arg.expr(), Contract::Pure, Default::default())
                }
            }
            SemExprData::Closure { body, .. } => {
                self.infer_expr(body, Contract::Move, Default::default())
            }
            // todo: macro arguments
            SemExprData::Sorry { .. } => (),
            // todo: macro arguments
            SemExprData::Todo { .. } => (),
            // todo: macro arguments
            SemExprData::Unreachable { .. } => (),
            SemExprData::NestedBlock { stmts, .. } => {
                self.infer_stmts(stmts, contract, site.clone())
            }
        }
    }

    fn infer_ritchie_parameter_argument_matches(
        &mut self,
        ritchie_parameter_argument_matches: &[SemaRitchieArgument],
    ) {
        for m in ritchie_parameter_argument_matches {
            match m {
                SemaRitchieArgument::Simple(param, arg) => {
                    self.infer_expr(arg.argument_expr_idx, param.contract, Default::default())
                }
                SemaRitchieArgument::Variadic(param, args) => {
                    for arg in args {
                        self.infer_expr(
                            arg.argument_expr_idx(),
                            param.contract(),
                            Default::default(),
                        )
                    }
                }
                SemaRitchieArgument::Keyed(param, arg) => {
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
        SynExprRootKind::PrimalTrait => None,
        SynExprRootKind::TraitInConstraint => None,
        SynExprRootKind::ReturnType => None,
        SynExprRootKind::PropsStructFieldType { .. } => None,
        SynExprRootKind::TupleStructFieldType => None,
        SynExprRootKind::BlockExpr => Some(Contract::Move),
        SynExprRootKind::ReturnExpr => Some(Contract::Move),
        SynExprRootKind::StaticExpr => Some(Contract::Move),
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
        SynExprRootKind::AssocTypeValue => None,
        SynExprRootKind::TypeAliasValue => None,
        SynExprRootKind::TypeVarDefault => None,
        SynExprRootKind::Effect => todo!(),
        SynExprRootKind::DefaultConstExclude => todo!(),
        SynExprRootKind::Dep => todo!(),
    }
}
