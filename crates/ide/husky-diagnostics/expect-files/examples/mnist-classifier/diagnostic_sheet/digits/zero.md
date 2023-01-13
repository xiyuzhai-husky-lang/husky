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
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [35:34, 35:35),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}