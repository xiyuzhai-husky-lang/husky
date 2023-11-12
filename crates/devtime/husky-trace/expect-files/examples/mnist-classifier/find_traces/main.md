[
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::zero`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_lazy_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_lazy_stmt_idx: Some(
                    9,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            5,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            15,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..8,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_lazy_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_lazy_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_lazy_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_lazy_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_lazy_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 3,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_lazy_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                    path_data: Return,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_lazy_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_lazy_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_lazy_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_lazy_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_lazy_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_lazy_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_lazy_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_lazy_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_lazy_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_lazy_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_lazy_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_lazy_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 3,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_lazy_stmt_idx: Some(
                    21,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::one`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 4,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    42,
                ),
                hir_lazy_stmt_idx: Some(
                    42,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    43,
                ),
                hir_lazy_stmt_idx: Some(
                    43,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    44,
                ),
                hir_lazy_stmt_idx: Some(
                    44,
                ),
                data: IfBranch {
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
                            2..5,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 24,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_lazy_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 24,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_lazy_stmt_idx: Some(
                    3,
                ),
                data: IfBranch {
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
                            1..2,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 24,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_lazy_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    path_data: ElseBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    44,
                ),
                hir_lazy_stmt_idx: Some(
                    44,
                ),
                data: ElseBranch {
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
                            16..42,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_lazy_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_lazy_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_lazy_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_lazy_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_lazy_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_lazy_stmt_idx: Some(
                    21,
                ),
                data: IfBranch {
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
                            5..9,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 3,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    22,
                ),
                hir_lazy_stmt_idx: Some(
                    22,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    23,
                ),
                hir_lazy_stmt_idx: Some(
                    23,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    24,
                ),
                hir_lazy_stmt_idx: Some(
                    24,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    25,
                ),
                hir_lazy_stmt_idx: Some(
                    25,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    26,
                ),
                hir_lazy_stmt_idx: Some(
                    26,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            227,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            231,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            9..15,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    27,
                ),
                hir_lazy_stmt_idx: Some(
                    27,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            334,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            338,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            15..16,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 3,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    28,
                ),
                hir_lazy_stmt_idx: Some(
                    28,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 4,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    29,
                ),
                hir_lazy_stmt_idx: Some(
                    29,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 5,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    30,
                ),
                hir_lazy_stmt_idx: Some(
                    30,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 6,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    31,
                ),
                hir_lazy_stmt_idx: Some(
                    31,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 7,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    32,
                ),
                hir_lazy_stmt_idx: Some(
                    32,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    33,
                ),
                hir_lazy_stmt_idx: Some(
                    33,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 4,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    34,
                ),
                hir_lazy_stmt_idx: Some(
                    34,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 8,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    35,
                ),
                hir_lazy_stmt_idx: Some(
                    35,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 9,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    36,
                ),
                hir_lazy_stmt_idx: Some(
                    36,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 10,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    37,
                ),
                hir_lazy_stmt_idx: Some(
                    37,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 11,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    38,
                ),
                hir_lazy_stmt_idx: Some(
                    38,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 5,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    39,
                ),
                hir_lazy_stmt_idx: Some(
                    39,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 6,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    40,
                ),
                hir_lazy_stmt_idx: Some(
                    40,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 3,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    41,
                ),
                hir_lazy_stmt_idx: Some(
                    41,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::six`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 6,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 7,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_lazy_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_lazy_stmt_idx: Some(
                    21,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    22,
                ),
                hir_lazy_stmt_idx: Some(
                    22,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    23,
                ),
                hir_lazy_stmt_idx: Some(
                    23,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    24,
                ),
                hir_lazy_stmt_idx: Some(
                    24,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    25,
                ),
                hir_lazy_stmt_idx: Some(
                    25,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            34,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            43,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            6..18,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_lazy_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_lazy_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_lazy_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 2,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_lazy_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 3,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_lazy_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 4,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_lazy_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_lazy_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_lazy_stmt_idx: Some(
                    13,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            106,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            110,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..5,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Let,
                    disambiguator: 5,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_lazy_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_lazy_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 1,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_lazy_stmt_idx: Some(
                    16,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            169,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            175,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            5..6,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 60,
                            },
                        ),
                    ),
                    path_data: Return,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 60,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_lazy_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    26,
                ),
                hir_lazy_stmt_idx: Some(
                    26,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    27,
                ),
                hir_lazy_stmt_idx: Some(
                    27,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            195,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            201,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            18..19,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 62,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 62,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_lazy_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    28,
                ),
                hir_lazy_stmt_idx: Some(
                    28,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: IfBranch,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    29,
                ),
                hir_lazy_stmt_idx: Some(
                    29,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            224,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            233,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            19..20,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: LazyStmt(
                        LazyStmtTracePath(
                            Id {
                                value: 64,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 0,
                },
                biological_parent: LazyStmt(
                    LazyStmtTrace(
                        Id {
                            value: 64,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_lazy_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    30,
                ),
                hir_lazy_stmt_idx: Some(
                    30,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::LazyStmt(
            LazyStmtTrace {
                path: LazyStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    path_data: Eval,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    31,
                ),
                hir_lazy_stmt_idx: Some(
                    31,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::three`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 9,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_eager_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_eager_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 6,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_eager_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 6,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_eager_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 7,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_eager_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 8,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_eager_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_eager_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::four`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 11,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 12,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 13,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_eager_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_eager_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_eager_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_eager_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_eager_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_eager_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_eager_stmt_idx: Some(
                    21,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    22,
                ),
                hir_eager_stmt_idx: Some(
                    22,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    23,
                ),
                hir_eager_stmt_idx: Some(
                    23,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    24,
                ),
                hir_eager_stmt_idx: Some(
                    24,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            72,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            81,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..15,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 5,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 6,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_eager_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 7,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_eager_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 8,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_eager_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 35,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 35,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_eager_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    25,
                ),
                hir_eager_stmt_idx: Some(
                    25,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::five`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 15,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 15,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::seven`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 16,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 16,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 17,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 17,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_eager_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_eager_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_eager_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_eager_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_eager_stmt_idx: Some(
                    15,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            21,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            25,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            4..8,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 58,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 58,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 58,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 58,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 58,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 58,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            45,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            49,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..4,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 58,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 58,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_eager_stmt_idx: Some(
                    16,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            79,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            83,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            8..11,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 59,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 59,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 59,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 59,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_eager_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_eager_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_eager_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_eager_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::eight`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 19,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            27,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            38,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            2..4,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 77,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 77,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            39,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            50,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..2,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 77,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 77,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 20,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::nine`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 21,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 22,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_eager_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_eager_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_eager_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_eager_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_eager_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_eager_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_eager_stmt_idx: Some(
                    21,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    22,
                ),
                hir_eager_stmt_idx: Some(
                    22,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    23,
                ),
                hir_eager_stmt_idx: Some(
                    23,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    24,
                ),
                hir_eager_stmt_idx: Some(
                    24,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            62,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            71,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..15,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 5,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 6,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_eager_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 7,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_eager_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 8,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_eager_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 92,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 92,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_eager_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 23,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    25,
                ),
                hir_eager_stmt_idx: Some(
                    25,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::two`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 24,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    20,
                ),
                hir_eager_stmt_idx: Some(
                    20,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    21,
                ),
                hir_eager_stmt_idx: Some(
                    21,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    22,
                ),
                hir_eager_stmt_idx: Some(
                    22,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    23,
                ),
                hir_eager_stmt_idx: Some(
                    23,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    24,
                ),
                hir_eager_stmt_idx: Some(
                    24,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    25,
                ),
                hir_eager_stmt_idx: Some(
                    25,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    26,
                ),
                hir_eager_stmt_idx: Some(
                    26,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    27,
                ),
                hir_eager_stmt_idx: Some(
                    27,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 6,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    28,
                ),
                hir_eager_stmt_idx: Some(
                    28,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    29,
                ),
                hir_eager_stmt_idx: Some(
                    29,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    30,
                ),
                hir_eager_stmt_idx: Some(
                    30,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    31,
                ),
                hir_eager_stmt_idx: Some(
                    31,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 7,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    32,
                ),
                hir_eager_stmt_idx: Some(
                    32,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    33,
                ),
                hir_eager_stmt_idx: Some(
                    33,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 8,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    34,
                ),
                hir_eager_stmt_idx: Some(
                    34,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    35,
                ),
                hir_eager_stmt_idx: Some(
                    35,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            93,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            97,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            1..15,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 4,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 5,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 6,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 7,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    11,
                ),
                hir_eager_stmt_idx: Some(
                    11,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 8,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    12,
                ),
                hir_eager_stmt_idx: Some(
                    12,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 9,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    13,
                ),
                hir_eager_stmt_idx: Some(
                    13,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 124,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 124,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    14,
                ),
                hir_eager_stmt_idx: Some(
                    14,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    36,
                ),
                hir_eager_stmt_idx: Some(
                    36,
                ),
                data: IfBranch {
                    if_regional_token: IfRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            222,
                        ),
                    },
                    eol_colon_regional_token: EolColonRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            226,
                        ),
                    },
                    stmts: SemaStmtIdxRange(
                        ArenaIdxRange(
                            15..20,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 125,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 125,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    15,
                ),
                hir_eager_stmt_idx: Some(
                    15,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 125,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 125,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    16,
                ),
                hir_eager_stmt_idx: Some(
                    16,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 125,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 2,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 125,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    17,
                ),
                hir_eager_stmt_idx: Some(
                    17,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 125,
                            },
                        ),
                    ),
                    data: Require,
                    disambiguator: 3,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 125,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    18,
                ),
                hir_eager_stmt_idx: Some(
                    18,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 125,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 125,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    19,
                ),
                hir_eager_stmt_idx: Some(
                    19,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 25,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    37,
                ),
                hir_eager_stmt_idx: Some(
                    37,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::major`,
                ),
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 26,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 27,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 27,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    data: ForBetween,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 27,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: ForBetween {
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
                            3..5,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 149,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 149,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 149,
                            },
                        ),
                    ),
                    data: IfBranch,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 149,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: IfBranch {
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
                            1..3,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 152,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 152,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 152,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 1,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 152,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 27,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    data: Let,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 28,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    data: ForBetween,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 28,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: ForBetween {
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
                            1..2,
                        ),
                    ),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: EagerStmt(
                        EagerStmtTracePath(
                            Id {
                                value: 156,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: EagerStmt(
                    EagerStmtTrace(
                        Id {
                            value: 156,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    data: Return,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 28,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 29,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 30,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 31,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 32,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::main`, `Val`),
                },
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 0,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    1,
                ),
                hir_eager_stmt_idx: Some(
                    1,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 1,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    2,
                ),
                hir_eager_stmt_idx: Some(
                    2,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 2,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    3,
                ),
                hir_eager_stmt_idx: Some(
                    3,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 3,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    4,
                ),
                hir_eager_stmt_idx: Some(
                    4,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 4,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    5,
                ),
                hir_eager_stmt_idx: Some(
                    5,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 5,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    6,
                ),
                hir_eager_stmt_idx: Some(
                    6,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 6,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    7,
                ),
                hir_eager_stmt_idx: Some(
                    7,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 7,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    8,
                ),
                hir_eager_stmt_idx: Some(
                    8,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 8,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    9,
                ),
                hir_eager_stmt_idx: Some(
                    9,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
    (
        Trace::EagerStmt(
            EagerStmtTrace {
                path: EagerStmtTracePath {
                    parent_path: ValItem(
                        ValItemTracePath(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: Eval,
                    disambiguator: 9,
                },
                biological_parent: ValItem(
                    ValItemTrace(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                sema_stmt_idx: SemaStmtIdx(
                    10,
                ),
                hir_eager_stmt_idx: Some(
                    10,
                ),
                data: BasicStmt,
            },
        ),
        (),
    ),
]