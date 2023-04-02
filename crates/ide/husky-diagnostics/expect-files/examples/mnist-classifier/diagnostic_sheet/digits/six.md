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
                range: [16:9, 16:20),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [31:67, 31:68),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [32:13, 32:24),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [41:9, 41:20),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [53:5, 53:16),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [17:25, 17:28),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [33:29, 33:32),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [37:25, 37:28),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [42:25, 42:28),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [50:25, 50:28),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [51:21, 51:24),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [54:21, 54:24),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [64:17, 64:20),
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
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [16:9, 21:11),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [27:17, 27:40),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [32:13, 36:15),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [38:36, 38:74),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [41:9, 48:11),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [53:5, 57:7),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [58:8, 58:22),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [59:17, 59:31),
            },
        ],
    },
}