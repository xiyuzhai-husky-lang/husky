DiagnosticSheet {
    [salsa id]: 28,
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
                range: [43:21, 43:26),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [47:21, 47:26),
            },
            Diagnostic {
                message: "entity tree error EntityTreeError::Original(OriginalEntityTreeError::NoSubentity)",
                severity: Error,
                range: [51:17, 51:22),
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
                range: [35:29, 35:52),
            },
        ],
    },
}