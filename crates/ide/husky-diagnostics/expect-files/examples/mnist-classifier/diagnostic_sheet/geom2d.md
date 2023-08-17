DiagnosticSheet {
    [salsa id]: 39,
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
                message: "Type Error: no method named `sgnx` for type `f32`",
                severity: Error,
                range: [56:21, 56:25),
            },
            Diagnostic {
                message: "Type Error: no field named cos_value in Vector2d",
                severity: Error,
                range: [56:43, 56:52),
            },
            Diagnostic {
                message: "Type Error: no method named `sgnx` for type `f32`",
                severity: Error,
                range: [63:27, 63:31),
            },
            Diagnostic {
                message: "Type Error: no method named `acos` for type `f32`",
                severity: Error,
                range: [77:84, 77:88),
            },
        ],
    },
}