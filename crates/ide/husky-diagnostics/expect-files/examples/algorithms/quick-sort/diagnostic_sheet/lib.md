DiagnosticSheet {
    [salsa id]: 24,
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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [31:33, 31:34),
            },
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [37:29, 37:30),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: no method named `len` for type `Slice t at MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) }`",
                severity: Error,
                range: [2:19, 2:22),
            },
            Diagnostic {
                message: "Type Error: no method named `swap` for type `Slice t at MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) }`",
                severity: Error,
                range: [26:17, 26:21),
            },
            Diagnostic {
                message: "Type Error: no method named `swap` for type `Slice t at MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) }`",
                severity: Error,
                range: [27:9, 27:13),
            },
        ],
    },
}