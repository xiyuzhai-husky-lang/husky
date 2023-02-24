DiagnosticSheet {
    [salsa id]: 17,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [30:1, 30:6),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [36:1, 36:6),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [3:41, 3:46),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [5:49, 5:54),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [5:62, 5:67),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [11:44, 11:49),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [11:57, 11:62),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [11:67, 11:72),
            },
        ],
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