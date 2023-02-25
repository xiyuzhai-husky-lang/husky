DiagnosticSheet {
    [salsa id]: 17,
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
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect `->`",
                severity: Error,
                range: [1:43, 1:44),
            },
            Diagnostic {
                message: "Syntax Error: expect `->`",
                severity: Error,
                range: [5:68, 5:69),
            },
            Diagnostic {
                message: "Syntax Error: expect `->`",
                severity: Error,
                range: [31:35, 31:36),
            },
            Diagnostic {
                message: "Syntax Error: expect `->`",
                severity: Error,
                range: [37:31, 37:32),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [2:15, 2:18),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [3:20, 3:23),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [7:17, 7:26),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [7:27, 7:30),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [8:24, 8:27),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [9:24, 9:27),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [20:9, 20:19),
            },
            Diagnostic {
                message: "OriginalLocalTermExpectationError::Todo: todo",
                severity: Error,
                range: [22:13, 22:23),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [26:13, 26:16),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term Slice $0",
                severity: Error,
                range: [27:5, 27:8),
            },
        ],
    },
}