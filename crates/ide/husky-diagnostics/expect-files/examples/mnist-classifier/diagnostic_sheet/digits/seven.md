DiagnosticSheet {
    [salsa id]: 32,
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
                range: [43:16, 43:33),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [47:16, 47:33),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [51:5, 51:22),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [31:5, 51:22),
            },
        ],
    },
}