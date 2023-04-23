DiagnosticSheet {
    [salsa id]: 30,
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
                range: [19:19, 19:44),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [20:17, 20:42),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [21:16, 21:41),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [34:13, 34:35),
            },
        ],
    },
}