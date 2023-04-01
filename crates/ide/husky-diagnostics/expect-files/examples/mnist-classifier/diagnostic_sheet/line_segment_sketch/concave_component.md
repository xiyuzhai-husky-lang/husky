DiagnosticSheet {
    [salsa id]: 36,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [84:11, 84:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [85:9, 85:16),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [9:15, 9:16),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [9:15, 9:16),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [13:13, 13:14),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [17:17, 17:18),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [20:23, 20:24),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [33:21, 33:22),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [43:21, 43:22),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [60:30, 60:31),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [93:9, 93:16),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [97:13, 97:18),
            },
        ],
    },
}