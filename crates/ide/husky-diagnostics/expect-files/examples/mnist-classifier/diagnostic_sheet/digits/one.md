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
                range: [20:28, 20:29),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [36:41, 36:42),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [42:103, 42:104),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [45:38, 45:39),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [120:18, 120:19),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [121:18, 121:19),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}