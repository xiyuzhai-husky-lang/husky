DiagnosticSheet {
    [salsa id]: 32,
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
                range: [21:5, 21:16),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [19:21, 19:25),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [22:21, 22:25),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [39:17, 39:21),
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
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [13:17, 13:36),
            },
            Diagnostic {
                message: "Type Error: TodoSuffix",
                severity: Error,
                range: [21:5, 27:7),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [34:13, 34:51),
            },
        ],
    },
}