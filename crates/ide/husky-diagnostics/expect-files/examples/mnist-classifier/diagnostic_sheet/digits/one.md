DiagnosticSheet {
    [salsa id]: 31,
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
                message: "Term Error: expected coersion",
                severity: Error,
                range: [4:44, 4:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [4:54, 4:60),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [4:62, 4:65),
            },
            Diagnostic {
                message: "Type Error: no field named rel_norm in TypeOntology(core::option::Option) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)",
                severity: Error,
                range: [45:13, 45:48),
            },
            Diagnostic {
                message: "Type Error: no field named angle_change in TypeOntology(core::option::Option) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)",
                severity: Error,
                range: [46:13, 46:52),
            },
            Diagnostic {
                message: "Type Error: no field named norm in TypeOntology(core::option::Option) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)",
                severity: Error,
                range: [68:17, 68:48),
            },
            Diagnostic {
                message: "Type Error: no field named rel_norm in TypeOntology(core::option::Option) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)",
                severity: Error,
                range: [69:17, 69:52),
            },
            Diagnostic {
                message: "Type Error: no field named angle_change in TypeOntology(core::option::Option) TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)",
                severity: Error,
                range: [70:17, 70:56),
            },
        ],
    },
}