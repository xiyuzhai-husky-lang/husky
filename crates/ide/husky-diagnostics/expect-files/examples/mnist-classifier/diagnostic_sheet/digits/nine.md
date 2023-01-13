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
                range: [35:40, 35:41),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [40:40, 40:41),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [41:40, 41:41),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [62:18, 62:19),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}