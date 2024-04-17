```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 6,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 54,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 54,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 14,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 64,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 14,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 16,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 14,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 15,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 66,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 17,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_lazy_stmt_idx: Some(
                        7,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..7,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 30,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_lazy_stmt_idx: Some(
                        0,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 31,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_lazy_stmt_idx: Some(
                        1,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 32,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_lazy_stmt_idx: Some(
                        2,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 33,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_lazy_stmt_idx: Some(
                        3,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 34,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_lazy_stmt_idx: Some(
                        4,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 35,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_lazy_stmt_idx: Some(
                        5,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 36,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 17,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_lazy_stmt_idx: Some(
                        6,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 18,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_lazy_stmt_idx: Some(
                        8,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 19,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_lazy_stmt_idx: Some(
                        9,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 20,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_lazy_stmt_idx: Some(
                        10,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 21,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_lazy_stmt_idx: Some(
                        11,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 22,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_lazy_stmt_idx: Some(
                        12,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 23,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_lazy_stmt_idx: Some(
                        13,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 24,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_lazy_stmt_idx: Some(
                        14,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 25,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_lazy_stmt_idx: Some(
                        15,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 26,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_lazy_stmt_idx: Some(
                        16,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 27,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_lazy_stmt_idx: Some(
                        17,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 28,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_lazy_stmt_idx: Some(
                        18,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 29,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 15,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_lazy_stmt_idx: Some(
                        19,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 1,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 55,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 55,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 37,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 67,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 37,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 39,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 37,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 6,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 4,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 38,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 68,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 40,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 38,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        41,
                    ),
                    hir_lazy_stmt_idx: Some(
                        41,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 41,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 38,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        42,
                    ),
                    hir_lazy_stmt_idx: Some(
                        42,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 42,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 38,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        43,
                    ),
                    hir_lazy_stmt_idx: Some(
                        43,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                25,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                31,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                1..4,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 44,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 42,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_lazy_stmt_idx: Some(
                        1,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 45,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 42,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_lazy_stmt_idx: Some(
                        2,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                44,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                50,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..1,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 46,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 42,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_lazy_stmt_idx: Some(
                        3,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                },
                            ),
                        },
                        essence: ElseBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 43,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 38,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        43,
                    ),
                    hir_lazy_stmt_idx: Some(
                        43,
                    ),
                    lazy_stmt_sketch: ElseBranch {
                        else_regional_token: ElseRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                60,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                61,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                15..41,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 47,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_lazy_stmt_idx: Some(
                        15,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 48,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_lazy_stmt_idx: Some(
                        16,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 49,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_lazy_stmt_idx: Some(
                        17,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 50,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_lazy_stmt_idx: Some(
                        18,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 51,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_lazy_stmt_idx: Some(
                        19,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 52,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        20,
                    ),
                    hir_lazy_stmt_idx: Some(
                        20,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                99,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                103,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                4..8,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 53,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        21,
                    ),
                    hir_lazy_stmt_idx: Some(
                        21,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 54,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        22,
                    ),
                    hir_lazy_stmt_idx: Some(
                        22,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 55,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        23,
                    ),
                    hir_lazy_stmt_idx: Some(
                        23,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 56,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        24,
                    ),
                    hir_lazy_stmt_idx: Some(
                        24,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 57,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        25,
                    ),
                    hir_lazy_stmt_idx: Some(
                        25,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                227,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                234,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                8..14,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 58,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        26,
                    ),
                    hir_lazy_stmt_idx: Some(
                        26,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                337,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                344,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                14..15,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 59,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        27,
                    ),
                    hir_lazy_stmt_idx: Some(
                        27,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 60,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        28,
                    ),
                    hir_lazy_stmt_idx: Some(
                        28,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 61,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        29,
                    ),
                    hir_lazy_stmt_idx: Some(
                        29,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 6,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 62,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        30,
                    ),
                    hir_lazy_stmt_idx: Some(
                        30,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 7,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 63,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        31,
                    ),
                    hir_lazy_stmt_idx: Some(
                        31,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 64,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        32,
                    ),
                    hir_lazy_stmt_idx: Some(
                        32,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 4,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 65,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        33,
                    ),
                    hir_lazy_stmt_idx: Some(
                        33,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 8,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 66,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        34,
                    ),
                    hir_lazy_stmt_idx: Some(
                        34,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 9,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 67,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        35,
                    ),
                    hir_lazy_stmt_idx: Some(
                        35,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 10,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 68,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        36,
                    ),
                    hir_lazy_stmt_idx: Some(
                        36,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 11,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 69,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        37,
                    ),
                    hir_lazy_stmt_idx: Some(
                        37,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 5,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 70,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        38,
                    ),
                    hir_lazy_stmt_idx: Some(
                        38,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 6,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 71,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        39,
                    ),
                    hir_lazy_stmt_idx: Some(
                        39,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: ElseBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 72,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 43,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        40,
                    ),
                    hir_lazy_stmt_idx: Some(
                        40,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 8,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 2,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 56,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 56,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 73,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 72,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 73,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 76,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 73,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 10,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 7,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 74,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 73,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 74,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 77,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 74,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 12,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 9,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 75,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 74,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 78,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_lazy_stmt_idx: Some(
                        19,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 79,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        20,
                    ),
                    hir_lazy_stmt_idx: Some(
                        20,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 80,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        21,
                    ),
                    hir_lazy_stmt_idx: Some(
                        21,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 81,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        22,
                    ),
                    hir_lazy_stmt_idx: Some(
                        22,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 82,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        23,
                    ),
                    hir_lazy_stmt_idx: Some(
                        23,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                33,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                42,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                5..17,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 89,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_lazy_stmt_idx: Some(
                        5,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 90,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_lazy_stmt_idx: Some(
                        6,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 91,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_lazy_stmt_idx: Some(
                        7,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 92,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_lazy_stmt_idx: Some(
                        8,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 93,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_lazy_stmt_idx: Some(
                        9,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 94,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_lazy_stmt_idx: Some(
                        10,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 95,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_lazy_stmt_idx: Some(
                        11,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 96,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_lazy_stmt_idx: Some(
                        12,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                105,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                112,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..4,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 97,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_lazy_stmt_idx: Some(
                        13,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 98,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_lazy_stmt_idx: Some(
                        14,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 99,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_lazy_stmt_idx: Some(
                        15,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                171,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                177,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                4..5,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 0,
                                },
                            ),
                        },
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 100,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 82,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_lazy_stmt_idx: Some(
                        16,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 83,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        24,
                    ),
                    hir_lazy_stmt_idx: Some(
                        24,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 84,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        25,
                    ),
                    hir_lazy_stmt_idx: Some(
                        25,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                197,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                203,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                17..18,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 1,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 101,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 84,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_lazy_stmt_idx: Some(
                        17,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 85,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        26,
                    ),
                    hir_lazy_stmt_idx: Some(
                        26,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: IfBranch,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 86,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        27,
                    ),
                    hir_lazy_stmt_idx: Some(
                        27,
                    ),
                    lazy_stmt_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                226,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                238,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                18..19,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::LazyStmt(
                                LazyStmtTracePathData {
                                    biological_parent_path: TracePath {
                                        data: TracePathData::ValItem(
                                            ValItemTracePathData {
                                                val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            },
                                        ),
                                    },
                                    essence: IfBranch,
                                    disambiguator: 2,
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 102,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 86,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_lazy_stmt_idx: Some(
                        18,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 87,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        28,
                    ),
                    hir_lazy_stmt_idx: Some(
                        28,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::LazyStmt(
                    LazyStmtTracePathData {
                        biological_parent_path: TracePath {
                            data: TracePathData::ValItem(
                                ValItemTracePathData {
                                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                },
                            ),
                        },
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: LazyStmt(
                LazyStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 88,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 75,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        29,
                    ),
                    hir_lazy_stmt_idx: Some(
                        29,
                    ),
                    lazy_stmt_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                    hir_lazy_expr_region: HirLazyExprRegion(
                        Id {
                            value: 3,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 57,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 57,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 103,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 77,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 103,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 105,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 103,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 16,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 12,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 104,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 78,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 106,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 107,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 108,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 109,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 110,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 111,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 112,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 113,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 114,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 115,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 116,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_eager_stmt_idx: Some(
                        10,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 117,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_eager_stmt_idx: Some(
                        11,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Let,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 118,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_eager_stmt_idx: Some(
                        12,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 119,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_eager_stmt_idx: Some(
                        13,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 7,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 120,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_eager_stmt_idx: Some(
                        14,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Require,
                        disambiguator: 8,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 121,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_eager_stmt_idx: Some(
                        15,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 104,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 122,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 104,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_eager_stmt_idx: Some(
                        16,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 14,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 58,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 58,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 123,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 82,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 123,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 127,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 123,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 20,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 16,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 124,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 84,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 124,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 128,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 124,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 18,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 125,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 85,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 125,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 129,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 125,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 24,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 20,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 126,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 86,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 130,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_eager_stmt_idx: Some(
                        14,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 131,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_eager_stmt_idx: Some(
                        15,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 132,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_eager_stmt_idx: Some(
                        16,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 133,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_eager_stmt_idx: Some(
                        17,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 134,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_eager_stmt_idx: Some(
                        18,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 135,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_eager_stmt_idx: Some(
                        19,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 136,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        20,
                    ),
                    hir_eager_stmt_idx: Some(
                        20,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 137,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        21,
                    ),
                    hir_eager_stmt_idx: Some(
                        21,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Require,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 138,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        22,
                    ),
                    hir_eager_stmt_idx: Some(
                        22,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 139,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        23,
                    ),
                    hir_eager_stmt_idx: Some(
                        23,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                81,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                90,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..14,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 141,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 142,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 143,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 144,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 145,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 146,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 147,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 148,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 149,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 150,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 151,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_eager_stmt_idx: Some(
                        10,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 7,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 152,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_eager_stmt_idx: Some(
                        11,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Require,
                        disambiguator: 8,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 153,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_eager_stmt_idx: Some(
                        12,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 139,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 154,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 139,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_eager_stmt_idx: Some(
                        13,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 126,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 140,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 126,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        24,
                    ),
                    hir_eager_stmt_idx: Some(
                        24,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 22,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 59,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 155,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 89,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 155,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 156,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 155,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 28,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 24,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 60,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 157,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 90,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 157,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 160,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 157,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 26,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 158,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 158,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 161,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 158,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 32,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 28,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 159,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 95,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 162,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_eager_stmt_idx: Some(
                        10,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 163,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_eager_stmt_idx: Some(
                        11,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 164,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_eager_stmt_idx: Some(
                        12,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                17,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                3..7,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 164,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 170,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 164,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 164,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 171,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 164,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 164,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 172,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 164,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                40,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                44,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..3,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 164,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 173,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 164,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 165,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_eager_stmt_idx: Some(
                        13,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                74,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                78,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                7..10,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 165,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 174,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 165,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 165,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 175,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 165,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 165,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 176,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 165,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 166,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_eager_stmt_idx: Some(
                        14,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 167,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_eager_stmt_idx: Some(
                        15,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 168,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_eager_stmt_idx: Some(
                        16,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 159,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 169,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 159,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_eager_stmt_idx: Some(
                        17,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 30,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 61,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 61,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 177,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 96,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 177,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 179,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 177,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 36,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 32,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 178,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 97,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 178,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 180,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 178,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 178,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 181,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 178,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                22,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                1..3,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 181,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 183,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 181,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                34,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..1,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 181,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 184,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 181,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 178,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 182,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 178,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 34,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 62,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 62,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 185,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 99,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 185,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 188,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 185,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 36,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 186,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 100,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 186,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 189,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 186,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 42,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 38,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 187,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 101,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 190,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_eager_stmt_idx: Some(
                        14,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 191,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_eager_stmt_idx: Some(
                        15,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 192,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_eager_stmt_idx: Some(
                        16,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 193,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_eager_stmt_idx: Some(
                        17,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 194,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_eager_stmt_idx: Some(
                        18,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 195,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_eager_stmt_idx: Some(
                        19,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 196,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        20,
                    ),
                    hir_eager_stmt_idx: Some(
                        20,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 197,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        21,
                    ),
                    hir_eager_stmt_idx: Some(
                        21,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                57,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                66,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..14,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 199,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 200,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 201,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 202,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 203,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 204,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 205,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 206,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 207,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 208,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 209,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_eager_stmt_idx: Some(
                        10,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 7,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 210,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_eager_stmt_idx: Some(
                        11,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Require,
                        disambiguator: 8,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 211,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_eager_stmt_idx: Some(
                        12,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 197,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 212,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 197,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_eager_stmt_idx: Some(
                        13,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 187,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 198,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 187,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        22,
                    ),
                    hir_eager_stmt_idx: Some(
                        22,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 40,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 63,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 63,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 213,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 104,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 213,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 215,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 213,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 46,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 42,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 214,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 108,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 216,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        19,
                    ),
                    hir_eager_stmt_idx: Some(
                        19,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 217,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        20,
                    ),
                    hir_eager_stmt_idx: Some(
                        20,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 218,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        21,
                    ),
                    hir_eager_stmt_idx: Some(
                        21,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 219,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        22,
                    ),
                    hir_eager_stmt_idx: Some(
                        22,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 220,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        23,
                    ),
                    hir_eager_stmt_idx: Some(
                        23,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 221,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        24,
                    ),
                    hir_eager_stmt_idx: Some(
                        24,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 222,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        25,
                    ),
                    hir_eager_stmt_idx: Some(
                        25,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 223,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        26,
                    ),
                    hir_eager_stmt_idx: Some(
                        26,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 224,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        27,
                    ),
                    hir_eager_stmt_idx: Some(
                        27,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 225,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        28,
                    ),
                    hir_eager_stmt_idx: Some(
                        28,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                69,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                73,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..14,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 228,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 229,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 230,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 231,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 232,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 233,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 234,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 235,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 236,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 237,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 7,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 238,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        10,
                    ),
                    hir_eager_stmt_idx: Some(
                        10,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 8,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 239,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        11,
                    ),
                    hir_eager_stmt_idx: Some(
                        11,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Let,
                        disambiguator: 9,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 240,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        12,
                    ),
                    hir_eager_stmt_idx: Some(
                        12,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 225,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 241,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 225,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        13,
                    ),
                    hir_eager_stmt_idx: Some(
                        13,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 226,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        29,
                    ),
                    hir_eager_stmt_idx: Some(
                        29,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                204,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                208,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                14..19,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 226,
                            },
                        ),
                        essence: Require,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 242,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 226,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        14,
                    ),
                    hir_eager_stmt_idx: Some(
                        14,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 226,
                            },
                        ),
                        essence: Require,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 243,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 226,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        15,
                    ),
                    hir_eager_stmt_idx: Some(
                        15,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 226,
                            },
                        ),
                        essence: Require,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 244,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 226,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        16,
                    ),
                    hir_eager_stmt_idx: Some(
                        16,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 226,
                            },
                        ),
                        essence: Require,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 245,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 226,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        17,
                    ),
                    hir_eager_stmt_idx: Some(
                        17,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 226,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 246,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 226,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        18,
                    ),
                    hir_eager_stmt_idx: Some(
                        18,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 214,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 227,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 214,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        30,
                    ),
                    hir_eager_stmt_idx: Some(
                        30,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 44,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::Submodule(
                    SubmoduleTracePathData {
                        submodule_item_path: SubmoduleItemPath(
                            ItemPathId(
                                Id {
                                    value: 7,
                                },
                            ),
                        ),
                    },
                ),
            },
            data: Submodule(
                SubmoduleTraceData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 247,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 109,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 247,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 254,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 247,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 50,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 46,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 248,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 110,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 248,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 255,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 248,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 248,
                            },
                        ),
                        essence: Let,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 256,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 248,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 248,
                            },
                        ),
                        essence: ForBetween,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 257,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 248,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: ForBetween {
                        for_regional_token: StmtForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                11,
                            ),
                        },
                        eol_colon_regional_token: Colon(
                            EolColonRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    19,
                                ),
                            },
                        ),
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                2..4,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 257,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 259,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 257,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 257,
                            },
                        ),
                        essence: IfBranch,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 260,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 257,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: IfBranch {
                        if_regional_token: IfRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                29,
                            ),
                        },
                        eol_colon_regional_token: EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                33,
                            ),
                        },
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..2,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 260,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 261,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 260,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 260,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 262,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 260,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 248,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 258,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 248,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 48,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 249,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 111,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 249,
                            },
                        ),
                        essence: Let,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 263,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 249,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 54,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 50,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 249,
                            },
                        ),
                        essence: ForBetween,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 264,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 249,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: ForBetween {
                        for_regional_token: StmtForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                6,
                            ),
                        },
                        eol_colon_regional_token: Colon(
                            EolColonRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    14,
                                ),
                            },
                        ),
                        stmts: SemaStmtIdxRange(
                            ArenaIdxRange(
                                0..1,
                            ),
                        ),
                    },
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 54,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 50,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 264,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 266,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 264,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 54,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 50,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 249,
                            },
                        ),
                        essence: Return,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 265,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 249,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 54,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 50,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 250,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 112,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 250,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 267,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 250,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 56,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 52,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 251,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 113,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 251,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 268,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 251,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 58,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 54,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 252,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 114,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 252,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 269,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 252,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 56,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 253,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 115,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 253,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 270,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 253,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 62,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 58,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist_classifier::main`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 3,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 0,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 271,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        0,
                    ),
                    hir_eager_stmt_idx: Some(
                        0,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 1,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 272,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        1,
                    ),
                    hir_eager_stmt_idx: Some(
                        1,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 2,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 273,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        2,
                    ),
                    hir_eager_stmt_idx: Some(
                        2,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 3,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 274,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        3,
                    ),
                    hir_eager_stmt_idx: Some(
                        3,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 4,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 275,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        4,
                    ),
                    hir_eager_stmt_idx: Some(
                        4,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 5,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 276,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        5,
                    ),
                    hir_eager_stmt_idx: Some(
                        5,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 6,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 277,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        6,
                    ),
                    hir_eager_stmt_idx: Some(
                        6,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 7,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 278,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        7,
                    ),
                    hir_eager_stmt_idx: Some(
                        7,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 8,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 279,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        8,
                    ),
                    hir_eager_stmt_idx: Some(
                        8,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
    (
        Trace {
            path: TracePath {
                data: TracePathData::EagerStmt(
                    EagerStmtTracePathData {
                        biological_parent_path: TracePath(
                            Id {
                                value: 3,
                            },
                        ),
                        essence: Eval,
                        disambiguator: 9,
                    },
                ),
            },
            data: EagerStmt(
                EagerStmtTraceData {
                    path: TracePath(
                        Id {
                            value: 280,
                        },
                    ),
                    biological_parent: Trace(
                        Id {
                            value: 3,
                        },
                    ),
                    sem_stmt_idx: SemaStmtIdx(
                        9,
                    ),
                    hir_eager_stmt_idx: Some(
                        9,
                    ),
                    eager_stmt_data_sketch: BasicStmt,
                    sem_expr_region: SemaExprRegion(
                        Id {
                            value: 64,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion(
                        Id {
                            value: 60,
                        },
                    ),
                },
            ),
        },
        (),
    ),
]
```