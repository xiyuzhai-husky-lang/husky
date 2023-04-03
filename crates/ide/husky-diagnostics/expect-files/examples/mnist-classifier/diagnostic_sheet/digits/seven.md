DiagnosticSheet {
    [salsa id]: 28,
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
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [35:29, 35:52),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [43:9, 43:26),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [47:9, 47:26),
            },
        ],
    },
}