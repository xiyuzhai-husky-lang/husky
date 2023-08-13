DiagnosticSheet {
    [salsa id]: 26,
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
                message: "Type Error: no method named `ctz` for type `r32`",
                severity: Error,
                range: [132:27, 132:30),
            },
            Diagnostic {
                message: "Type Error: missing argument",
                severity: Error,
                range: [133:28, 133:43),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `bool` under contract ``",
                severity: Error,
                range: [130:15, 130:28),
            },
            Diagnostic {
                message: "Type Error: no method named `max` for type `i32`",
                severity: Error,
                range: [56:31, 56:34),
            },
        ],
    },
}