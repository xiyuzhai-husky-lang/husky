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
                range: [16:18, 16:19),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [24:18, 24:19),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [30:18, 30:19),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}