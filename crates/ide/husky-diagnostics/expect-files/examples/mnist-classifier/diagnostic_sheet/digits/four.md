DiagnosticSheet {
    [salsa id]: 29,
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
                range: [86:16, 86:32),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [98:5, 98:21),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [20:5, 98:21),
            },
        ],
    },
}