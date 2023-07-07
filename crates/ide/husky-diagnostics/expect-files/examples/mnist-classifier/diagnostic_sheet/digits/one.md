DiagnosticSheet {
    [salsa id]: 31,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Term Error: expected function type",
                severity: Error,
                range: [6:17, 6:28),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [7:5, 11:7),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [14:9, 17:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [29:13, 32:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [37:9, 43:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [44:9, 48:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [55:13, 60:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [61:13, 65:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [67:13, 72:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [79:9, 83:11),
            },
        ],
    },
}