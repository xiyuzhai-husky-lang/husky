use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
use husky_entity_kind::ritchie::RitchieItemKind;
#[allow(unused_imports)]
use husky_sema_expr::emit_note_on_sema_expr_codespan;
use husky_sema_expr::{SemaExprData, SemaExprIdx, SemaRitchieParameterArgumentMatch};
use husky_syn_expr::SynExprRootKind;
use husky_term_prelude::{
    ritchie::{RitchieClosureKind, RitchieTypeKind},
    TermContract,
};

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
        contract: TermContract,
        mut site: SemaPlaceContractSite,
    ) {
        if let Some(place) = self.place(expr) {
            site.set(place, contract)
        }
        match *expr.data(self.sema_expr_region_data().sema_expr_arena()) {
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
                lopd,
                opr,
                ref dispatch,
                opr_regional_token_idx,
                ropd,
            } => todo!(),
            SemaExprData::Be {
                src,
                be_regional_token_idx,
                ref target,
            } => todo!(),
            SemaExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd_sema_expr_idx,
            } => todo!(),
            SemaExprData::Suffix {
                opd_sema_expr_idx,
                opr,
                opr_regional_token_idx,
            } => todo!(),
            SemaExprData::Unveil {
                opd_sema_expr_idx, ..
            } => self.infer_expr(opd_sema_expr_idx, TermContract::Move, site.clone()),
            SemaExprData::Unwrap {
                opd_sema_expr_idx, ..
            } => self.infer_expr(opd_sema_expr_idx, TermContract::At, site.clone()),
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
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieParameterArgumentMatch::Simple(param, arg) => self.infer_expr(
                            arg.argument_expr_idx,
                            param.contract,
                            Default::default(),
                        ),
                        SemaRitchieParameterArgumentMatch::Variadic(_, _) => {
                            todo!()
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
            SemaExprData::Ritchie { .. } => {
                todo!()
            }
            SemaExprData::Field {
                owner_sema_expr_idx,
                owner_ty,
                dot_regional_token_idx,
                ident_token,
                ref dispatch,
            } => todo!(),
            SemaExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::MethodFnCall {
                self_argument_sema_expr_idx,
                self_contract,
                dot_regional_token_idx,
                ident_token,
                ref dispatch,
                ref template_arguments,
                lpar_regional_token_idx,
                ref ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
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
            SemaExprData::Delimitered {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemaExprData::Index {
                owner_sema_expr_idx,
                lbox_regional_token_idx,
                ref index_sema_list_items,
                rbox_regional_token_idx,
                ref index_dynamic_dispatch,
            } => todo!(),
            SemaExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::NewList { ref items, .. } => {
                for item in items {
                    self.infer_expr(item.sema_expr_idx, TermContract::Move, Default::default())
                }
            }
            SemaExprData::BoxColonList { ref items, .. } => {
                todo!()
            }
            SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemaExprData::Block { stmts } => self.infer_stmts(stmts, contract, site.clone()),
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SemaExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                ref parameter_obelisks,
                rvert_regional_token,
                return_ty,
                body,
            } => todo!(),
            SemaExprData::Sorry { regional_token_idx } => todo!(),
            SemaExprData::Todo { regional_token_idx } => todo!(),
            SemaExprData::Unreachable { regional_token_idx } => todo!(),
            SemaExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
        }
        self.set_expr_site(expr, site)
    }
}

fn root_contract(root_kind: SynExprRootKind) -> Option<TermContract> {
    match root_kind {
        SynExprRootKind::SelfType => todo!(),
        SynExprRootKind::Trait => todo!(),
        SynExprRootKind::ReturnType => None,
        SynExprRootKind::PropsStructFieldType { ident_token } => todo!(),
        SynExprRootKind::TupleStructFieldType => todo!(),
        SynExprRootKind::BlockExpr => Some(TermContract::Move),
        SynExprRootKind::ReturnExpr => todo!(),
        SynExprRootKind::Condition => todo!(),
        SynExprRootKind::ExplicitParameterDefaultValue { ty_syn_expr_idx } => todo!(),
        SynExprRootKind::FieldBindInitialValue { ty_syn_expr_idx } => todo!(),
        SynExprRootKind::ConstantImplicitParameterType => todo!(),
        SynExprRootKind::ExplicitParameterType => todo!(),
        SynExprRootKind::HtmlArgumentExpr => todo!(),
        SynExprRootKind::LetStmtType => todo!(),
        SynExprRootKind::LetStmtInitialValue => todo!(),
        SynExprRootKind::Snippet => todo!(),
        SynExprRootKind::ValExpr => todo!(),
        SynExprRootKind::EvalExpr => todo!(),
        SynExprRootKind::AssocTypeTerm => todo!(),
        SynExprRootKind::TypeAliasTypeTerm => todo!(),
    }
}
