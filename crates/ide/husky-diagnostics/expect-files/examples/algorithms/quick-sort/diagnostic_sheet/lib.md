DiagnosticSheet {
    [salsa id]: 24,
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
                message: "Term Error: expected coersion from `List i32` to `Slice _t` under contract `move `",
                severity: Error,
                range: [33:16, 33:17),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `List Ref 'static str` to `Slice _t` under contract `move `",
                severity: Error,
                range: [39:16, 39:20),
            },
        ],
    },
}