DiagnosticSheet {
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [101:5, 101:29),
            },
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [102:5, 102:29),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [103:1, 103:2),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}