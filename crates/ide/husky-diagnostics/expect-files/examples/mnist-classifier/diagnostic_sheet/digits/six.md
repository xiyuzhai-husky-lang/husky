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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [16:9, 16:20),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [32:13, 32:24),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [41:9, 41:20),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [53:5, 53:16),
            },
        ],
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
                range: [11:24, 11:41),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [16:9, 21:11),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [22:29, 22:55),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [26:22, 26:47),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [27:17, 27:40),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [32:13, 36:15),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [38:36, 38:74),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [41:9, 48:11),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [53:5, 57:7),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [58:8, 58:22),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [59:17, 59:31),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [37:13, 37:28),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [50:13, 50:28),
            },
            Diagnostic {
                message: "type path mismatch: expect core::basic::unit, but got mnist::MnistLabel instead",
                severity: Error,
                range: [51:9, 51:24),
            },
        ],
    },
}