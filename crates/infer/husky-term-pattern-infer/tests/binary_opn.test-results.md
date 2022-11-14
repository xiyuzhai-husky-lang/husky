# Binary Opn

## Test#0

input

```husky
0 + 0
```

output

```husky
raw_expr_arena = IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Literal(
                    Integer(
                        0,
                    ),
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: None,
        },
        RawExpr {
            variant: Atom(
                Literal(
                    Integer(
                        0,
                    ),
                ),
            ),
            range: [1:5, 1:6),
            base_scope_result: None,
        },
        RawExpr {
            variant: Opn {
                opn_variant: Binary(
                    PureClosed(
                        Add,
                    ),
                ),
                opds: 0..2,
            },
            range: [1:1, 1:6),
            base_scope_result: None,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [
                IntegerLiteral(0),
                IntegerType(UnresolvedTermIdx(0)),
                IntegerLiteral(1),
                IntegerType(UnresolvedTermIdx(2)),
            ],
        },
    },
    expr_results: ArenaMap {
        data: [
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        2,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                3,
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        None,
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
        ],
    },
};
```

## Test#1

input

```husky
0i32 + 0
```

output

```husky
raw_expr_arena = IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Literal(
                    I32(
                        0,
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: None,
        },
        RawExpr {
            variant: Atom(
                Literal(
                    Integer(
                        0,
                    ),
                ),
            ),
            range: [1:8, 1:9),
            base_scope_result: None,
        },
        RawExpr {
            variant: Opn {
                opn_variant: Binary(
                    PureClosed(
                        Add,
                    ),
                ),
                opds: 0..2,
            },
            range: [1:1, 1:9),
            base_scope_result: None,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [
                IntegerLiteral(1),
                IntegerType(UnresolvedTermIdx(0)),
            ],
        },
    },
    expr_results: ArenaMap {
        data: [
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Resolved(
                                    Atom(Literal(I32(0))),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Resolved(
                            Atom(Entity { path: `i32` }),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        None,
                    ),
                    ty: Ok(
                        Resolved(
                            Atom(Entity { path: `i32` }),
                        ),
                    ),
                },
            ),
        ],
    },
};
```

## Test#2

input

```husky
0. + 0
```

output

```husky
raw_expr_arena = IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Literal(
                    Float(
                        OrderedFloat(
                            0.0,
                        ),
                    ),
                ),
            ),
            range: [1:1, 1:3),
            base_scope_result: None,
        },
        RawExpr {
            variant: Atom(
                Literal(
                    Integer(
                        0,
                    ),
                ),
            ),
            range: [1:6, 1:7),
            base_scope_result: None,
        },
        RawExpr {
            variant: Opn {
                opn_variant: Binary(
                    PureClosed(
                        Add,
                    ),
                ),
                opds: 0..2,
            },
            range: [1:1, 1:7),
            base_scope_result: None,
        },
    ],
};

ty_infer_sheet = TermPatternInferSheet {
    term_patt_itr: TermPatternInterner {
        patterns: [],
        unresolved_registry: UnresolvedTermRegistry {
            terms: [
                FloatLiteral(
                    0,
                ),
                FloatType(
                    UnresolvedTermIdx(
                        0,
                    ),
                ),
                IntegerLiteral(1),
                IntegerType(UnresolvedTermIdx(2)),
            ],
        },
    },
    expr_results: ArenaMap {
        data: [
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        0,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        Some(
                            ConstExprPatternItd {
                                term: Unresolved(
                                    UnresolvedTermIdx(
                                        2,
                                    ),
                                ),
                                opt_substitution_ctx_idx: None,
                            },
                        ),
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                3,
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TermPatternInferEntry {
                    const_expr: Ok(
                        None,
                    ),
                    ty: Ok(
                        Unresolved(
                            UnresolvedTermIdx(
                                1,
                            ),
                        ),
                    ),
                },
            ),
        ],
    },
};
```
