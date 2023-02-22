DiagnosticSheet {
    [salsa id]: 39,
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
                range: [8:31, 8:36),
            },
        ],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: original `todo` in term List ConnectedComponent",
                severity: Error,
                range: [13:13, 13:33),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List ConnectedComponent",
                severity: Error,
                range: [22:13, 22:33),
            },
            Diagnostic {
                message: "TypeError: todo",
                severity: Error,
                range: [34:5, 34:42),
            },
            Diagnostic {
                message: "TypeError: todo",
                severity: Error,
                range: [37:5, 37:49),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}