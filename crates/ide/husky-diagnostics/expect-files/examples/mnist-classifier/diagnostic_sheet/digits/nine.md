DiagnosticSheet {
    [salsa id]: 30,
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
                message: "Type Error: no method named `displacement` for type `Option Leash ConcaveComponent`",
                severity: Error,
                range: [18:38, 18:50),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `Option Leash ConcaveComponent`",
                severity: Error,
                range: [31:27, 31:39),
            },
            Diagnostic {
                message: "Type Error: no field named angle_change in Option Leash ConcaveComponent",
                severity: Error,
                range: [32:17, 32:39),
            },
        ],
    },
}