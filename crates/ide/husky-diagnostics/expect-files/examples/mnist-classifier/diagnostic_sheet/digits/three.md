DiagnosticSheet {
    [salsa id]: 34,
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
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [37:5, 37:22),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [17:5, 37:22),
            },
        ],
    },
}