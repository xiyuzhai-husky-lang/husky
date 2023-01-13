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
                range: [67:9, 67:15),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [69:9, 69:15),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [87:9, 87:15),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}