DiagnosticSheet {
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
                range: [33:27, 33:44),
            },
        ],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: original `todo` in term Ref TermLiteral::EvalLifetime List ConcaveComponent",
                severity: Error,
                range: [32:22, 32:40),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List TermCurry { variance: Invariant, x: Term(`Ref TermLiteral::EvalLifetime ConcaveComponent`), y: Term(`Option f32`) }",
                severity: Error,
                range: [35:13, 35:22),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [38:12, 38:28),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [32:5, 38:45),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [11:9, 14:20),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [17:9, 20:20),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [23:9, 26:20),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}