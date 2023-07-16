DiagnosticSheet {
    [salsa id]: 32,
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
                range: [4:43, 4:66),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [12:43, 12:59),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [12:61, 12:79),
            },
            Diagnostic {
                message: "Type Error: no method named `end_tangent` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`TypeOntology(core::option::Option)`), argument: EtherealTerm(`TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent)`), shift: 0 })`",
                severity: Error,
                range: [39:61, 39:72),
            },
        ],
    },
}