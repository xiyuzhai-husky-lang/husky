DiagnosticSheet {
    [salsa id]: 18,
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
                message: "unresolved term",
                severity: Error,
                range: [3:35, 3:36),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [8:38, 8:39),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [9:33, 9:34),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [13:33, 13:34),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [32:17, 32:54),
            },
            Diagnostic {
                message: "Type Error: AmbiguateListExpr",
                severity: Error,
                range: [38:20, 38:73),
            },
        ],
    },
}