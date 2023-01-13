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
                range: [63:5, 63:11),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [71:5, 71:11),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [107:5, 107:11),
            },
            Diagnostic {
                message: "Syntax Error: unexpected keyword",
                severity: Error,
                range: [20:9, 20:15),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}