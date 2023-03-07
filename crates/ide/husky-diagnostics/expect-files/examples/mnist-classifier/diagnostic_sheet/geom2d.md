DiagnosticSheet {
    [salsa id]: 34,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [101:5, 101:29),
            },
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [102:5, 102:29),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [103:1, 103:2),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [49:23, 49:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [66:23, 66:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [68:24, 68:25),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [70:23, 70:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [77:27, 77:28),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect `}`",
                severity: Error,
                range: [101:5, 101:8),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [],
    },
}