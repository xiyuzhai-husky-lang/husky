DiagnosticSheet {
    [salsa id]: 26,
    item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
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
                message: "Type Error: no method named `collect_refs` for type `Vec RawContour`",
                severity: Error,
                range: [34:50, 34:62),
            },
            Diagnostic {
                message: "Type Error: no method named `span` for type `r32`",
                severity: Error,
                range: [56:48, 56:52),
            },
            Diagnostic {
                message: "Type Error: no method named `span` for type `r32`",
                severity: Error,
                range: [63:42, 63:46),
            },
            Diagnostic {
                message: "Type Error: no method named `co` for type `r32`",
                severity: Error,
                range: [79:41, 79:43),
            },
            Diagnostic {
                message: "Type Error: no method named `co` for type `r32`",
                severity: Error,
                range: [82:41, 82:43),
            },
            Diagnostic {
                message: "Type Error: no method named `span` for type `r32`",
                severity: Error,
                range: [104:48, 104:52),
            },
            Diagnostic {
                message: "Type Error: no method named `right_mass` for type `r32`",
                severity: Error,
                range: [115:48, 115:58),
            },
        ],
    },
}