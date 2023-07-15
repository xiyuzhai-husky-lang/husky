DiagnosticSheet {
    [salsa id]: 36,
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
                range: [4:44, 4:57),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::num::f32 instead",
                severity: Error,
                range: [8:5, 8:27),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::num::f32 instead",
                severity: Error,
                range: [7:5, 8:27),
            },
            Diagnostic {
                message: "Type Error: no method named `displacement` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`TypeOntology(core::option::Option)`), argument: EtherealTerm(`TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)`), shift: 0 })`",
                severity: Error,
                range: [17:43, 17:55),
            },
        ],
    },
}