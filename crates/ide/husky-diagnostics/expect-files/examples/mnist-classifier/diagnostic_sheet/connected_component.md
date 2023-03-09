DiagnosticSheet {
    [salsa id]: 21,
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
                range: [115:11, 115:12),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [116:9, 116:13),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [23:11, 23:24),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [126:41, 126:54),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [133:28, 133:41),
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
                message: "unresolved term",
                severity: Error,
                range: [20:11, 20:14),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [119:32, 119:33),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [119:43, 119:44),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [120:32, 120:33),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [120:43, 120:44),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [123:28, 123:29),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [123:39, 123:40),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [127:22, 127:24),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [140:44, 140:45),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [141:71, 141:72),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [146:34, 146:35),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [149:80, 149:81),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [32:43, 32:45),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [68:39, 68:40),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [73:36, 73:37),
            },
        ],
    },
}