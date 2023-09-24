use super::*;
use husky_syn_defn::HasDefns;
use husky_syn_expr::{
    OriginalSynExprError, PrincipalEntityPathExpr, SynExpr, SynExprError, SynExprRegion,
    SynExprResult, SynStmt,
};
use salsa::DebugWithDb;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct ExprDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn expr_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> ExprDiagnosticSheet {
    let mut sheet_collector = ModuleDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(defns)) =
        (db.ranged_token_sheet(module_path), module_path.defns(db))
    {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for defn in defns.iter().copied() {
            let decl = defn.syn_decl(db);
            if let Some(syn_expr_region) = decl.syn_expr_region(db) {
                sheet_collector
                    .region_collector(syn_expr_region)
                    .collect_expr_diagnostics(syn_expr_region);
            }
            if let Some(syn_expr_region) = defn.syn_expr_region(db) {
                sheet_collector
                    .region_collector(syn_expr_region)
                    .collect_expr_diagnostics(syn_expr_region);
            }
        }
    }
    let diagnostics = sheet_collector.finish();
    ExprDiagnosticSheet::new(db, diagnostics)
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_syn_expr_result<T>(&mut self, result: &SynExprResult<T>) {
        if let Err(SynExprError::Original(e)) = result {
            self.visit_atom(e)
        }
    }
    fn collect_expr_diagnostics(&mut self, syn_expr_region: SynExprRegion) {
        let expr_region_data = syn_expr_region.data(self.db());
        for expr in expr_region_data.expr_arena().data() {
            match expr {
                SynExpr::Err(SynExprError::Original(e)) => self.visit_atom(e),
                // self.visit_atom(e),
                _ => (),
            }
        }
        for stmt in expr_region_data.stmt_arena().data() {
            match stmt {
                SynStmt::Let {
                    let_token,
                    let_variables_pattern,
                    assign_token,
                    initial_value,
                } => {
                    self.visit_syn_expr_result(let_variables_pattern);
                    self.visit_syn_expr_result(assign_token);
                }
                SynStmt::Return {
                    return_token,
                    result,
                } => {}
                SynStmt::Require {
                    require_token,
                    condition,
                } => {}
                SynStmt::Assert {
                    assert_token,
                    condition,
                } => {}
                SynStmt::Break { break_token } => {}
                SynStmt::Eval { eol_semicolon, .. } => {}
                SynStmt::ForBetween {
                    for_token,
                    particulars,
                    eol_colon,
                    block,
                    ..
                } => {
                    self.visit_syn_expr_result(&particulars.range);
                    self.visit_syn_expr_result(eol_colon);
                }
                SynStmt::ForIn {
                    for_token,
                    condition,
                    eol_colon,
                    block,
                } => todo!(),
                SynStmt::ForExt {
                    forext_token,
                    particulars,
                    eol_colon,
                    block,
                } => {
                    self.visit_syn_expr_result(eol_colon);
                    // todo: handle errors in particulars
                }
                SynStmt::While {
                    while_token,
                    condition,
                    eol_colon,
                    block,
                } => {
                    self.visit_syn_expr_result(condition);
                    self.visit_syn_expr_result(eol_colon);
                }
                SynStmt::DoWhile {
                    do_token,
                    while_token,
                    condition,
                    eol_colon,
                    block,
                } => {
                    self.visit_syn_expr_result(condition);
                    self.visit_syn_expr_result(eol_colon);
                }
                SynStmt::IfElse {
                    if_branch,
                    elif_branches,
                    else_branch,
                } => {
                    self.visit_syn_expr_result(&if_branch.condition);
                    self.visit_syn_expr_result(&if_branch.eol_colon);
                    for elif_branch in elif_branches {
                        self.visit_syn_expr_result(&elif_branch.condition);
                        self.visit_syn_expr_result(&elif_branch.eol_colon);
                    }
                    if let Some(else_branch) = else_branch {
                        self.visit_syn_expr_result(&else_branch.eol_colon);
                    }
                }
                SynStmt::Match {
                    match_token,
                    match_expr,
                    eol_with_token,
                    ref case_branches,
                } => {
                    self.visit_syn_expr_result(match_expr);
                    self.visit_syn_expr_result(eol_with_token);
                    for case_branch in case_branches {
                        self.visit_syn_expr_result(&case_branch.case_pattern);
                        self.visit_syn_expr_result(&case_branch.heavy_arrow_token);
                        self.visit_syn_expr_result(&case_branch.stmts);
                    }
                }
            }
        }
        for item_path_expr in expr_region_data.principal_item_path_expr_arena().data() {
            match item_path_expr {
                PrincipalEntityPathExpr::Root { .. } => (),
                PrincipalEntityPathExpr::Subitem {
                    ident_token, path, ..
                } => {
                    match ident_token {
                        Err(SynExprError::Original(e)) => self.visit_atom(e),
                        _ => (),
                    }
                    match path {
                        Err(SynExprError::Original(e)) => self.visit_atom(e),
                        _ => (),
                    }
                }
            }
        }
    }
}

impl Diagnose for OriginalSynExprError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        match self {
            OriginalSynExprError::MismatchingBracket { .. } => {
                format!("Syntax Error: mismatching bracket")
            }
            OriginalSynExprError::ExpectedRightAngleBracket { .. } => {
                format!("Syntax Error: expected `>`")
            }
            OriginalSynExprError::ExpectedRightCurlyBrace(_) => {
                format!("Syntax Error: expected `}}`")
            }
            OriginalSynExprError::ExpectedIdent(_) => format!("Syntax Error: expected identifier"),
            OriginalSynExprError::ExpectedColon(_) => format!("Syntax Error: expected `:`"),
            OriginalSynExprError::ExpectedRightParenthesis(_) => {
                format!("Syntax Error: expected `)`")
            }
            OriginalSynExprError::NoMatchingBra { .. } => {
                format!("Syntax Error: no matching bracket")
            }
            OriginalSynExprError::ExpectedIdentAfterDot { .. } => {
                format!("Syntax Error: expected identifier after dot")
            }
            OriginalSynExprError::NoLeftOperandForBinaryOperator { .. } => {
                format!("Syntax Error: no left operand for binary operator")
            }
            OriginalSynExprError::NoRightOperandForBinaryOperator { .. } => {
                format!("Syntax Error: no right operand for binary operator")
            }
            OriginalSynExprError::NoOperandForPrefixOperator { .. } => {
                format!("Syntax Error:no operand for prefix operator")
            }
            OriginalSynExprError::ExpectedItemBeforeComma { .. } => {
                format!("Syntax Error: expected item before `,`")
            }
            OriginalSynExprError::ExpectedItemBeforeBe { .. } => {
                format!("Syntax Error: expected item before `be`")
            }
            OriginalSynExprError::ExpectedLetPattern(_) => {
                format!("Syntax Error: expected variable pattern after `let`")
            }
            OriginalSynExprError::ExpectedBePattern(_) => {
                format!("Syntax Error: expected pattern after `be`")
            }
            OriginalSynExprError::ExpectedCasePattern(_) => {
                format!("Syntax Error: expected pattern after `|`")
            }
            OriginalSynExprError::ExpectedAssign(_) => format!("Syntax Error: expected `=`"),
            OriginalSynExprError::ExpectedInitialValue(_) => {
                format!("Syntax Error: expected initial value")
            }
            OriginalSynExprError::UnexpectedKeyword(_) => {
                format!("Syntax Error: unexpected keyword")
            }
            OriginalSynExprError::ExpectedResult(_) => format!("Syntax Error: expected result"),
            OriginalSynExprError::ExpectedCondition(_) => {
                format!("Syntax Error: expected condition")
            }
            OriginalSynExprError::ExpectedMatchExpr(_) => {
                format!("Syntax Error: expected match expression")
            }
            OriginalSynExprError::ExpectedEolWithInMatchHead(_) => {
                format!("Syntax Error: expected end of line `with` in match head")
            }
            OriginalSynExprError::ExpectedForExpr(_) => format!("Syntax Error: expected for expr"),
            OriginalSynExprError::ExpectedParameterPattern(_) => {
                format!("Syntax Error: expected paramter pattern")
            }
            OriginalSynExprError::ExpectedEolColon(_) => {
                format!("Syntax Error: expected `:` at end of line")
            }
            OriginalSynExprError::ExpectedIdentAfterModifier(_, _) => {
                format!("Syntax Error: expected identifier after `mut`")
            }
            OriginalSynExprError::ExpectedConstantImplicitParameterType(_) => {
                format!("Syntax Error: expected constant implicit parameter type")
            }
            OriginalSynExprError::ExpectedHeavyArrowAfterCasePattern(token_stream_state) => {
                format!("Syntax Error: expected `=>` after case pattern")
            }
            OriginalSynExprError::ExpectedBlock(_) => format!("Syntax Error: expected block"),
            OriginalSynExprError::UnterminatedList { .. } => {
                format!("Syntax Error: unterminated list")
            }
            OriginalSynExprError::UnterminatedFunctionCallKeyedArgumentList {
                bra_regional_token_idx,
            } => {
                format!("Syntax Error: unterminated function call keyed argument list")
            }
            OriginalSynExprError::UnterminatedMethodCallKeyedArgumentList {
                bra_regional_token_idx,
            } => {
                format!("Syntax Error: unterminated method call keyed argument list")
            }
            OriginalSynExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
            OriginalSynExprError::UnrecognizedIdent {
                regional_token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unrecognized identifier")
            }
            OriginalSynExprError::UnresolvedSubitem {
                regional_token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unresolved subitem")
            }
            OriginalSynExprError::ExpectedLetVariablesType(_) => {
                format!("Syntax Error: expected let variables type")
            }
            OriginalSynExprError::ExpectedFieldType(_) => {
                format!("Syntax Error: expected field type")
            }
            OriginalSynExprError::ExpectedParameterType(_) => {
                format!("Syntax Error: expected parameter type")
            }
            OriginalSynExprError::SelfTypeNotAllowed(_) => {
                format!("Syntax Error: SelfTypeNotAllowed")
            }
            OriginalSynExprError::SelfValueNotAllowed(_) => {
                format!("Syntax Error: SelfValueNotAllowed")
            }
            OriginalSynExprError::ExpectedIdentAfterDot { .. } => {
                format!("Syntax Error: ExpectedIdentAfterDot")
            }
            OriginalSynExprError::ExpectedExprBeforeDot { .. } => {
                format!("Syntax Error: ExpectedExprBeforeDot")
            }
            OriginalSynExprError::HtmlTodo(_) => {
                format!("Syntax Error: HtmlTodo")
            }
            OriginalSynExprError::ExpectedValueForFieldBindInitialization(_) => {
                format!("Syntax Error: ExpectedValueForFieldBindInitialization")
            }
            OriginalSynExprError::ExpectedFunctionIdentAfterOpeningHtmlBra(_) => {
                format!("Syntax Error: ExpectedFunctionIdentAfterOpeningHtmlBra")
            }
            OriginalSynExprError::UnexpectedLeftCurlyBrace(_) => {
                format!("Syntax Error: UnexpectedLeftCurlyBrace")
            }
            OriginalSynExprError::ExpectedTraits(_) => todo!(),
            OriginalSynExprError::ExpectedTypeAfterLightArrow { light_arrow_token } => todo!(),
            OriginalSynExprError::ExpectedExplicitParameterDefaultValue(_) => todo!(),
            OriginalSynExprError::ExpectedTypeTermForAssociatedType(_) => todo!(),
            OriginalSynExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
            OriginalSynExprError::EntityTree {
                regional_token_idx,
                error,
            } => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        ctx.tokens_text_range(self.regional_token_idx_range())
    }
}
