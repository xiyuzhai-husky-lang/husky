DiagnosticSheet {
    [salsa id]: 26,
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
                range: [16:20, 16:38),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [25:38, 25:63),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [27:17, 27:39),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [29:25, 29:50),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [33:17, 33:39),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [38:9, 38:25),
            },
        ],
    },
}