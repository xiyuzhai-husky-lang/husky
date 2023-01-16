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
                range: [92:9, 92:19),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [92:21, 92:25),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [104:5, 104:15),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [104:17, 104:21),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}