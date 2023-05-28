DiagnosticSheet {
    [salsa id]: 30,
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
                message: "Type Error: no field named max_hole_ilen in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [34:13, 34:52),
            },
            Diagnostic {
                message: "Type Error: no field named upper_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [38:28, 38:64),
            },
            Diagnostic {
                message: "Type Error: no field named lower_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [38:67, 38:103),
            },
            Diagnostic {
                message: "Type Error: no field named upper_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [45:28, 45:64),
            },
            Diagnostic {
                message: "Type Error: no field named lower_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [45:67, 45:103),
            },
        ],
    },
}