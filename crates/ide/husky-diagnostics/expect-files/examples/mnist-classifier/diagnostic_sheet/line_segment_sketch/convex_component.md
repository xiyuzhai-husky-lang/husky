DiagnosticSheet {
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [5:21, 5:22),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [5:21, 5:22),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}