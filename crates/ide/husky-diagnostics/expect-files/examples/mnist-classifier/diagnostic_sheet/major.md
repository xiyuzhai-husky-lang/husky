DiagnosticSheet {
    [salsa id]: 43,
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
                range: [4:31, 4:36),
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
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [10:28, 10:51),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [14:12, 14:36),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [19:16, 19:39),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [27:5, 27:46),
            },
        ],
    },
}