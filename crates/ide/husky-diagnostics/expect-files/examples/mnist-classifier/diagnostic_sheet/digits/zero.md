DiagnosticSheet {
    [salsa id]: 36,
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
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [21:5, 26:7),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [19:16, 19:32),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [38:5, 38:21),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got mnist::MnistLabel instead",
                severity: Error,
                range: [11:5, 38:21),
            },
        ],
    },
}