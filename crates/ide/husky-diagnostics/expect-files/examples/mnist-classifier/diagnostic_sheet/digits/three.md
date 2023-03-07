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
                range: [10:27, 10:43),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [23:18, 23:28),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [26:17, 26:18),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [27:15, 27:16),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [28:14, 28:15),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [33:12, 33:13),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [35:25, 35:26),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [36:26, 36:27),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [37:18, 37:19),
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
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [48:12, 48:13),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [53:12, 53:13),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [58:12, 58:13),
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
                message: "Type Error: TodoScopeResolution",
                severity: Error,
                range: [44:5, 44:22),
            },
        ],
    },
}