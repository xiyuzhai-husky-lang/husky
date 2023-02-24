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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [10:27, 10:43),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [23:18, 23:28),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [44:5, 44:15),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [44:17, 44:22),
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
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [11:17, 11:41),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [11:43, 11:65),
            },
            Diagnostic {
                message: "Type Error: TodoScopeResolution",
                severity: Error,
                range: [44:5, 44:22),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List ConcaveComponent",
                severity: Error,
                range: [24:13, 24:37),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List ConcaveComponent",
                severity: Error,
                range: [25:13, 25:37),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [48:14, 48:16),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [50:6, 50:8),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [53:14, 53:16),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [55:6, 55:8),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [58:14, 58:16),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Ref 'eval ConcaveComponent",
                severity: Error,
                range: [60:6, 60:8),
            },
        ],
    },
}