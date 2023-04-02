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
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [86:21, 86:25),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [98:17, 98:21),
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
                range: [75:17, 75:44),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [81:17, 81:44),
            },
        ],
    },
}