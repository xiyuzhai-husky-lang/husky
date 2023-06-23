DiagnosticSheet {
    [salsa id]: 33,
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
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [16:9, 20:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [31:13, 34:15),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [39:9, 45:11),
            },
            Diagnostic {
                message: "Type Error: cannot unveil",
                severity: Error,
                range: [50:5, 53:7),
            },
        ],
    },
}