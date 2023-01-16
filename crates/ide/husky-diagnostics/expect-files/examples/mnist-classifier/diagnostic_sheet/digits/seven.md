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
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [51:9, 51:19),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [51:21, 51:26),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [55:9, 55:19),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [55:21, 55:26),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [59:5, 59:15),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [59:17, 59:22),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}