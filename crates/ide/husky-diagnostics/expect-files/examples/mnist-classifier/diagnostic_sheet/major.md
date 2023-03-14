DiagnosticSheet {
    [salsa id]: 41,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "SymbolNotAccessible",
                severity: Error,
                range: [4:33, 4:50),
            },
        ],
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
                range: [8:31, 8:36),
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
                range: [34:5, 34:42),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [37:5, 37:49),
            },
        ],
    },
}