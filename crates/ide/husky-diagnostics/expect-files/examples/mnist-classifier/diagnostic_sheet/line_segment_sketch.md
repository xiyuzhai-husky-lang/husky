DiagnosticSheet {
    [salsa id]: 44,
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
                message: "Type Error: no method named `min` for type `i32`",
                severity: Error,
                range: [122:22, 122:25),
            },
            Diagnostic {
                message: "Type Error: no field named start in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [186:53, 186:73),
            },
            Diagnostic {
                message: "Type Error: no field named end in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [186:75, 186:88),
            },
            Diagnostic {
                message: "Type Error: no field named end in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [195:8, 195:36),
            },
            Diagnostic {
                message: "Type Error: no field named start in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [199:13, 199:43),
            },
            Diagnostic {
                message: "Type Error: missing argument",
                severity: Error,
                range: [27:9, 27:72),
            },
        ],
    },
}