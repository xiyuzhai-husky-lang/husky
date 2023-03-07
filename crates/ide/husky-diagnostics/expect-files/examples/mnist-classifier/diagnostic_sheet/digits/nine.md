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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [11:20, 11:36),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [14:27, 14:43),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [17:17, 17:27),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [18:13, 18:20),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [19:13, 19:19),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [20:19, 20:20),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [24:19, 24:20),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [26:25, 26:26),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [28:23, 28:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [33:37, 33:38),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [36:27, 36:28),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [37:23, 37:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [42:15, 42:16),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [46:9, 46:19),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [46:21, 46:25),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [58:5, 58:15),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [58:17, 58:21),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [61:12, 61:13),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [66:12, 66:13),
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
                range: [46:9, 46:25),
            },
            Diagnostic {
                message: "Type Error: TodoScopeResolution",
                severity: Error,
                range: [58:5, 58:21),
            },
        ],
    },
}