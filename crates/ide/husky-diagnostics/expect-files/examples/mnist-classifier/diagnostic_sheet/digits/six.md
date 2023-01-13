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
                range: [34:41, 34:42),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [54:35, 54:36),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [64:32, 64:33),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [78:13, 78:14),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}