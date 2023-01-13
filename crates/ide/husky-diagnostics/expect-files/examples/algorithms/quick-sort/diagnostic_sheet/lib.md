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
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [34:5, 34:11),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [40:5, 40:11),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}