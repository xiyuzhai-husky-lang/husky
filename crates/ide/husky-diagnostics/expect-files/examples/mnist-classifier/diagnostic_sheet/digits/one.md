DiagnosticSheet {
    [salsa id]: 31,
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
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [10:17, 10:18),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [16:21, 16:22),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [31:25, 31:26),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [42:22, 42:23),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [47:21, 47:22),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [59:25, 59:26),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [64:25, 64:26),
            },
            Diagnostic {
                message: "Syntax Error: expect item before `,`",
                severity: Error,
                range: [71:25, 71:26),
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
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [7:5, 11:7),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [14:9, 17:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [29:13, 32:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [37:9, 43:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [44:9, 48:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [55:13, 60:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [61:13, 65:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [67:13, 72:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [79:9, 83:11),
            },
        ],
    },
}