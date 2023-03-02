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
        diagnostics: [],
    },
}