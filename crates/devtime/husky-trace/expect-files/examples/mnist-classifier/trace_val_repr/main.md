[
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::zero`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 0,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 1,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 4,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 4,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 37,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 10,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 15,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 20,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 25,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 35,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 37,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 14,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 14,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 19,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 19,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 24,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 24,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 34,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 34,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 34,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 36,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 36,
                        },
                    ),
                ),
                opn: ValOpn::Return,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 36,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 51,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 51,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 51,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 57,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 57,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 57,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 63,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 63,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 63,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 88,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 88,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 88,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::one`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 92,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 2,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 303,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 100,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 104,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 108,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 116,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 117,
                                },
                            ),
                        ],
                    },
                    Branch {
                        condition: None,
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 121,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 124,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 169,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 177,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 191,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 235,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 256,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 259,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 284,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 303,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 115,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 108,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 111,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 115,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 303,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 100,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 104,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 108,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 116,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 117,
                                },
                            ),
                        ],
                    },
                    Branch {
                        condition: None,
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 121,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 124,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 157,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 169,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 177,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 191,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 235,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 256,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 259,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 284,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 287,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 299,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 302,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 303,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 120,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 120,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 120,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 123,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 123,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 123,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 156,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 135,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 137,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 141,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 146,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 154,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 156,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 168,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 168,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 168,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 234,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 191,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 192,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 202,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 217,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 227,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 234,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 255,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 235,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 236,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 255,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 258,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 258,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 258,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 286,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 286,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 286,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 298,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 298,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 298,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 301,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 301,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 301,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::six`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 304,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 305,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 306,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 308,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 308,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 308,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 314,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 314,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 314,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 381,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 320,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 325,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 330,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 349,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 363,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 373,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 379,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 348,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 348,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 348,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 362,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 349,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 350,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 354,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 357,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 360,
                                },
                            ),
                            ValRepr(
                                Id {
                                    value: 362,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 378,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 373,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 376,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 378,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 380,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 380,
                        },
                    ),
                ),
                opn: ValOpn::Return,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 380,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
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
        Some(
            ValRepr {
                [salsa id]: 393,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 386,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 389,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 392,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 392,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 392,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 402,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 402,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 402,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 416,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 403,
                        },
                    ),
                ),
                opn: ValOpn::Branches,
                arguments: [
                    Branch {
                        condition: Some(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        stmts: [
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ],
                    },
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 415,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 415,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 415,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        Some(
            ValRepr {
                [salsa id]: 419,
                val_domain_repr: StmtNotReturned(
                    ValRepr(
                        Id {
                            value: 419,
                        },
                    ),
                ),
                opn: ValOpn::Require,
                arguments: [
                    Ordinary(
                        ValRepr(
                            Id {
                                value: 419,
                            },
                        ),
                    ),
                ],
                caching_class: Stmt,
            },
        ),
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::three`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 421,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 422,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::four`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 423,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 424,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 425,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 426,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::five`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 427,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::seven`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 428,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 429,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 430,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::eight`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 431,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 432,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::nine`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 433,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 434,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 435,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::digits::two`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 436,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 437,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::Submodule(
            SubmoduleTrace {
                submodule_path: SubmodulePath(
                    `mnist_classifier::major`,
                ),
            },
        ),
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 20,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 5,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 94,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 438,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 439,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 77,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 38,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
    ),
    (
        Trace::ValItem(
            ValItemTrace {
                path: ValItemTracePath {
                    val_item_path: FugitivePath(`mnist_classifier::main`, `Val`),
                },
            },
        ),
        Some(
            ValRepr {
                [salsa id]: 440,
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::main`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
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
        None,
    ),
]