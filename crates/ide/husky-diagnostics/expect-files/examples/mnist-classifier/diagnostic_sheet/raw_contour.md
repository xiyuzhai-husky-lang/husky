DiagnosticSheet {
    [salsa id]: 45,
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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [172:35, 172:47),
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
                message: "Type Error: no method named `last_bits` for type `FluffyTerm::EntityPath(TermEntityPath::TypeOntology(TypePath(`core::raw_bits::r32`, `Extern`)))`",
                severity: Error,
                range: [102:74, 102:83),
            },
            Diagnostic {
                message: "Type Error: no method named `last` for type `FluffyTerm::Solid(SolidTerm(1))`",
                severity: Error,
                range: [219:33, 219:37),
            },
            Diagnostic {
                message: "Type Error: no method named `last` for type `FluffyTerm::Solid(SolidTerm(1))`",
                severity: Error,
                range: [226:33, 226:37),
            },
            Diagnostic {
                message: "Type Error: no method named `last` for type `FluffyTerm::Solid(SolidTerm(1))`",
                severity: Error,
                range: [233:33, 233:37),
            },
            Diagnostic {
                message: "Type Error: no method named `pop` for type `FluffyTerm::Solid(SolidTerm(1))`",
                severity: Error,
                range: [264:25, 264:28),
            },
        ],
    },
}