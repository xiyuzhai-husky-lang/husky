DiagnosticSheet {
    [salsa id]: 38,
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
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [15:60, 15:72),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::raw_contour::RawContour)`))`",
                severity: Error,
                range: [21:60, 21:72),
            },
        ],
    },
}