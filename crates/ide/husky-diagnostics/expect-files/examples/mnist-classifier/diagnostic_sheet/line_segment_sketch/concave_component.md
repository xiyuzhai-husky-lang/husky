DiagnosticSheet {
    [salsa id]: 34,
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
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
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
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval LineSegmentSketch",
                severity: Error,
                range: [89:13, 89:32),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [92:37, 92:56),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [96:45, 96:64),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval LineSegmentSketch",
                severity: Error,
                range: [100:17, 100:36),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval LineSegmentSketch",
                severity: Error,
                range: [101:17, 101:36),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [55:16, 55:27),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [56:13, 56:24),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [57:13, 57:24),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [64:9, 64:20),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}