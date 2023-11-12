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
            Val {
                [salsa id]: 0,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 1,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 2,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 3,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 4,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 5,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 6,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 7,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 8,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 9,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 10,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 11,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 12,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 13,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 14,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 15,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 16,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 17,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 18,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 19,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 20,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 21,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 22,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 23,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                ),
                arguments: [],
            },
        ),
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
            Val {
                [salsa id]: 24,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 25,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 26,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 27,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 28,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 29,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 30,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                ),
                arguments: [],
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
            Val {
                [salsa id]: 31,
                domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist_classifier::main`, `Val`),
                ),
                arguments: [],
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