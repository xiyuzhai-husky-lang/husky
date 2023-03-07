DiagnosticSheet {
    [salsa id]: 24,
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
                range: [9:25, 9:41),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [16:34, 16:50),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [19:32, 19:48),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [22:17, 22:27),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [57:19, 57:20),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [70:20, 70:21),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [72:25, 72:26),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [74:23, 74:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [79:38, 79:39),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [82:27, 82:28),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [83:23, 83:24),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [88:15, 88:16),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [92:9, 92:19),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [92:21, 92:25),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [104:5, 104:15),
            },
            Diagnostic {
                message: "Syntax Error: unresolved subentity",
                severity: Error,
                range: [104:17, 104:21),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [107:12, 107:13),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [112:12, 112:13),
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
                range: [92:9, 92:25),
            },
            Diagnostic {
                message: "Type Error: TodoScopeResolution",
                severity: Error,
                range: [104:5, 104:21),
            },
        ],
    },
}