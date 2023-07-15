DiagnosticSheet {
    [salsa id]: 27,
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
                range: [4:44, 4:53),
            },
            Diagnostic {
                message: "Type Error: no method named `first` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`TypeOntology(core::slice::CyclicSliceLeashed)`), argument: EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentStroke)`), shift: 0 })`",
                severity: Error,
                range: [29:28, 29:33),
            },
            Diagnostic {
                message: "Type Error: no method named `first` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`TypeOntology(core::slice::CyclicSliceLeashed)`), argument: EtherealTerm(`TypeOntology(mnist_classifier::line_segment_sketch::LineSegmentStroke)`), shift: 0 })`",
                severity: Error,
                range: [29:58, 29:63),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::num::f32 instead",
                severity: Error,
                range: [30:5, 30:36),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::num::f32 instead",
                severity: Error,
                range: [28:5, 30:36),
            },
        ],
    },
}