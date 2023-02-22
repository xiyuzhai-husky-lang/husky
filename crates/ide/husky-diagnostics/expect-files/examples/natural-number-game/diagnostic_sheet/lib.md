DiagnosticSheet {
    [salsa id]: 41,
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside impl",
                severity: Error,
                range: [7:5, 7:24),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside impl",
                severity: Error,
                range: [8:5, 8:45),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [2:1, 2:7),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [3:1, 3:12),
            },
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [10:8, 10:9),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [11:5, 11:7),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [6:17, 6:18),
            },
        ],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}