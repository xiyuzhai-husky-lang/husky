# Variable

## Test#0

input

```husky
x
```

output

```husky
raw_expr_arena = Arena {
    data: [
        Expr {
            variant: Atom(
                Symbol(
                    Symbol {
                        ident: Custom(
                            Identifier(
                                "x",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: Uncertain,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [],
        },
    },
    var_results: {},
    expr_results: ArenaMap {
        data: [
            Some(
                ExprTermPatternInferResults {
                    const_expr: Err(
                        TermPatternInferError(
                            TermPatternInferErrorInternal {
                                source: Original(
                                    IdentUnrecognized {
                                        ident: Custom(
                                            Identifier(
                                                "x",
                                            ),
                                        ),
                                    },
                                ),
                                range: [1:1, 1:2),
                            },
                        ),
                    ),
                    ty: Err(
                        TermPatternInferError(
                            TermPatternInferErrorInternal {
                                source: Derived(
                                    TermPatternInferError(
                                        TermPatternInferError(
                                            TermPatternInferErrorInternal {
                                                source: Original(
                                                    IdentUnrecognized {
                                                        ident: Custom(
                                                            Identifier(
                                                                "x",
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                range: [1:1, 1:2),
                                            },
                                        ),
                                    ),
                                ),
                                range: [1:1, 1:2),
                            },
                        ),
                    ),
                },
            ),
        ],
    },
};
```
