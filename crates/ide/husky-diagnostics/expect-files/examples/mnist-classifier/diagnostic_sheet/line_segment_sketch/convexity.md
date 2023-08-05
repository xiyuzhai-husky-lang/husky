DiagnosticSheet {
    [salsa id]: 42,
    item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
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
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [8:68, 8:69),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [11:8, 11:48),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [13:68, 13:69),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [16:13, 16:98),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [22:13, 22:96),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract `move `",
                severity: Error,
                range: [23:16, 23:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract `move `",
                severity: Error,
                range: [25:16, 25:55),
            },
        ],
    },
}