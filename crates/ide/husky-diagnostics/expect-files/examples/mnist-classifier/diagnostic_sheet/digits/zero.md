DiagnosticSheet {
    [salsa id]: 36,
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
                message: "Type Error: no field named bounding_box in Option Leash RawContour",
                severity: Error,
                range: [32:13, 32:36),
            },
            Diagnostic {
                message: "Type Error: no field named bounding_box in Option Leash RawContour",
                severity: Error,
                range: [32:46, 32:69),
            },
        ],
    },
}