DiagnosticSheet {
    [salsa id]: 38,
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
                message: "Type Error: no method named `pop_with_largest_opt_f32` for type `List Leash ConcaveComponent`",
                severity: Error,
                range: [37:29, 37:53),
            },
            Diagnostic {
                message: "Type Error: no method named `max` for type `f32`",
                severity: Error,
                range: [13:25, 13:28),
            },
            Diagnostic {
                message: "Type Error: no method named `max` for type `f32`",
                severity: Error,
                range: [19:25, 19:28),
            },
            Diagnostic {
                message: "Type Error: no method named `max` for type `f32`",
                severity: Error,
                range: [25:25, 25:28),
            },
        ],
    },
}