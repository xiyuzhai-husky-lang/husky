DiagnosticSheet {
    [salsa id]: 35,
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
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [35:19, 35:39),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [36:20, 36:40),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `usize` under contract ``",
                severity: Error,
                range: [37:19, 37:39),
            },
        ],
    },
}