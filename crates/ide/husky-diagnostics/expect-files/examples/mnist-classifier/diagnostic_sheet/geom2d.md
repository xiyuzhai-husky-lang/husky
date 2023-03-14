DiagnosticSheet {
    [salsa id]: 35,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [101:5, 101:29),
            },
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [102:5, 102:29),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect `}`",
                severity: Error,
                range: [101:5, 101:8),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: no method named `norm` for type `LocalTerm::Resolved(Term(`TypeOntology(mnist_classifier::geom2d::Vector2d)`))`",
                severity: Error,
                range: [68:32, 68:36),
            },
        ],
    },
}