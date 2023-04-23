DiagnosticSheet {
    [salsa id]: 25,
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
                range: [20:13, 20:36),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [21:13, 21:36),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [64:22, 64:54),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [73:40, 73:70),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [75:17, 75:44),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [77:25, 77:55),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [81:17, 81:44),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [86:9, 86:25),
            },
        ],
    },
}