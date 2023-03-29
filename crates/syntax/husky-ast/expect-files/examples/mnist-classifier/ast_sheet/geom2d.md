Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        3..3,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            AssociatedFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `from_i_shift28`,
                        token_idx: TokenIdx(
                            18,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        19,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        1..2,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `vector`,
                        token_idx: TokenIdx(
                            50,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        51,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        2..3,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `to`,
                        token_idx: TokenIdx(
                            67,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        68,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        3..4,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `norm`,
                        token_idx: TokenIdx(
                            95,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        96,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        4..5,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `dist`,
                        token_idx: TokenIdx(
                            123,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        124,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        11..11,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        12..12,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        13..13,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        14..14,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        15..16,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    body: ArenaIdxRange(
                        17..18,
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 16,
                    elif_branches: ArenaIdxRange(
                        17..17,
                    ),
                    else_branch: Some(
                        18,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        19..20,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        33,
                    ),
                    body: ArenaIdxRange(
                        21..21,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: ArenaIdxRange(
                        21..22,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 20,
                    elif_branches: ArenaIdxRange(
                        21..21,
                    ),
                    else_branch: Some(
                        22,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        35,
                    ),
                    body: ArenaIdxRange(
                        25..25,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        44,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        43,
                    ),
                    body: ArenaIdxRange(
                        26..27,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        46,
                    ),
                    body: ArenaIdxRange(
                        28..28,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        45,
                    ),
                    body: ArenaIdxRange(
                        28..29,
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 27,
                    elif_branches: ArenaIdxRange(
                        28..28,
                    ),
                    else_branch: Some(
                        29,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        42,
                    ),
                    body: ArenaIdxRange(
                        30..31,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        48,
                    ),
                    body: ArenaIdxRange(
                        32..32,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        49,
                    ),
                    body: ArenaIdxRange(
                        32..32,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        47,
                    ),
                    body: ArenaIdxRange(
                        32..34,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        37,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        38,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        39,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        40,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        41,
                    ),
                    body: ArenaIdxRange(
                        26..26,
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 31,
                    elif_branches: ArenaIdxRange(
                        32..32,
                    ),
                    else_branch: Some(
                        34,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        10..11,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `point`,
                        token_idx: TokenIdx(
                            173,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        174,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        11..12,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `to`,
                        token_idx: TokenIdx(
                            191,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        192,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        12..13,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `norm`,
                        token_idx: TokenIdx(
                            219,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        220,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        13..14,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `dot`,
                        token_idx: TokenIdx(
                            247,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        248,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        14..15,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `cross`,
                        token_idx: TokenIdx(
                            272,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        273,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    body: ArenaIdxRange(
                        23..25,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `angle`,
                        token_idx: TokenIdx(
                            297,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        298,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        34,
                    ),
                    body: ArenaIdxRange(
                        25..26,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `rotation_direction_to`,
                        token_idx: TokenIdx(
                            366,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        367,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        36,
                    ),
                    body: ArenaIdxRange(
                        35..41,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `angle_to`,
                        token_idx: TokenIdx(
                            386,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        387,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        53,
                    ),
                    body: ArenaIdxRange(
                        49..49,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        54,
                    ),
                    body: ArenaIdxRange(
                        49..49,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        55,
                    ),
                    body: ArenaIdxRange(
                        49..49,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        56,
                    ),
                    body: ArenaIdxRange(
                        49..49,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        57,
                    ),
                    body: ArenaIdxRange(
                        49..49,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        59,
                    ),
                    body: ArenaIdxRange(
                        54..54,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        60,
                    ),
                    body: ArenaIdxRange(
                        54..54,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        52,
                    ),
                    body: ArenaIdxRange(
                        49..54,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `relative_range`,
                        token_idx: TokenIdx(
                            503,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        504,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        58,
                    ),
                    body: ArenaIdxRange(
                        54..56,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `relative_point`,
                        token_idx: TokenIdx(
                            566,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        567,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        64,
                    ),
                    body: ArenaIdxRange(
                        58..58,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        66,
                    ),
                    body: ArenaIdxRange(
                        59..59,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        68,
                    ),
                    body: ArenaIdxRange(
                        60..60,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        70,
                    ),
                    body: ArenaIdxRange(
                        61..61,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        72,
                    ),
                    body: ArenaIdxRange(
                        62..62,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        74,
                    ),
                    body: ArenaIdxRange(
                        63..63,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        63,
                    ),
                    body: ArenaIdxRange(
                        58..59,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `relative_bounding_box`,
                        token_idx: TokenIdx(
                            613,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        614,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        65,
                    ),
                    body: ArenaIdxRange(
                        59..60,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `relative_point`,
                        token_idx: TokenIdx(
                            648,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        649,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        67,
                    ),
                    body: ArenaIdxRange(
                        60..61,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `xmin`,
                        token_idx: TokenIdx(
                            683,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        684,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        69,
                    ),
                    body: ArenaIdxRange(
                        61..62,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `xmax`,
                        token_idx: TokenIdx(
                            695,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        696,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        71,
                    ),
                    body: ArenaIdxRange(
                        62..63,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `ymin`,
                        token_idx: TokenIdx(
                            707,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        708,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        73,
                    ),
                    body: ArenaIdxRange(
                        63..64,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `ymax`,
                        token_idx: TokenIdx(
                            719,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        720,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        78,
                    ),
                    body: ArenaIdxRange(
                        70..70,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        80,
                    ),
                    body: ArenaIdxRange(
                        71..71,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        82,
                    ),
                    body: ArenaIdxRange(
                        72..72,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        84,
                    ),
                    body: ArenaIdxRange(
                        73..73,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        77,
                    ),
                    body: ArenaIdxRange(
                        70..71,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `xmin`,
                        token_idx: TokenIdx(
                            747,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        748,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        79,
                    ),
                    body: ArenaIdxRange(
                        71..72,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `xmax`,
                        token_idx: TokenIdx(
                            759,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        760,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        81,
                    ),
                    body: ArenaIdxRange(
                        72..73,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `ymin`,
                        token_idx: TokenIdx(
                            771,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        772,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        83,
                    ),
                    body: ArenaIdxRange(
                        73..74,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::geom2d`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `ymax`,
                        token_idx: TokenIdx(
                            783,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        784,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Point2d`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        5..10,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `RelativePoint2d`,
                        token_idx: TokenIdx(
                            144,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        145,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Vector2d`,
                        token_idx: TokenIdx(
                            157,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        158,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        41..49,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        50,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ClosedRange`,
                        token_idx: TokenIdx(
                            488,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        489,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        51,
                    ),
                    body: ArenaIdxRange(
                        56..58,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        61,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `BoundingBox`,
                        token_idx: TokenIdx(
                            596,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        597,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        62,
                    ),
                    body: ArenaIdxRange(
                        64..70,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        75,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `RelativeBoundingBox`,
                        token_idx: TokenIdx(
                            732,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        733,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        76,
                    ),
                    body: ArenaIdxRange(
                        74..78,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            78..89,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..3,
            ),
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..5,
            ),
            ArenaIdxRange(
                5..10,
            ),
            ArenaIdxRange(
                10..10,
            ),
            ArenaIdxRange(
                10..10,
            ),
            ArenaIdxRange(
                10..10,
            ),
            ArenaIdxRange(
                10..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..12,
            ),
            ArenaIdxRange(
                12..12,
            ),
            ArenaIdxRange(
                12..13,
            ),
            ArenaIdxRange(
                13..13,
            ),
            ArenaIdxRange(
                13..14,
            ),
            ArenaIdxRange(
                14..14,
            ),
            ArenaIdxRange(
                14..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..16,
            ),
            ArenaIdxRange(
                17..17,
            ),
            ArenaIdxRange(
                17..18,
            ),
            ArenaIdxRange(
                19..20,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..22,
            ),
            ArenaIdxRange(
                23..25,
            ),
            ArenaIdxRange(
                25..25,
            ),
            ArenaIdxRange(
                25..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..27,
            ),
            ArenaIdxRange(
                28..28,
            ),
            ArenaIdxRange(
                28..29,
            ),
            ArenaIdxRange(
                30..31,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..34,
            ),
            ArenaIdxRange(
                35..41,
            ),
            ArenaIdxRange(
                41..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..49,
            ),
            ArenaIdxRange(
                49..54,
            ),
            ArenaIdxRange(
                54..54,
            ),
            ArenaIdxRange(
                54..54,
            ),
            ArenaIdxRange(
                54..56,
            ),
            ArenaIdxRange(
                56..58,
            ),
            ArenaIdxRange(
                58..58,
            ),
            ArenaIdxRange(
                58..58,
            ),
            ArenaIdxRange(
                58..59,
            ),
            ArenaIdxRange(
                59..59,
            ),
            ArenaIdxRange(
                59..60,
            ),
            ArenaIdxRange(
                60..60,
            ),
            ArenaIdxRange(
                60..61,
            ),
            ArenaIdxRange(
                61..61,
            ),
            ArenaIdxRange(
                61..62,
            ),
            ArenaIdxRange(
                62..62,
            ),
            ArenaIdxRange(
                62..63,
            ),
            ArenaIdxRange(
                63..63,
            ),
            ArenaIdxRange(
                63..64,
            ),
            ArenaIdxRange(
                64..70,
            ),
            ArenaIdxRange(
                70..70,
            ),
            ArenaIdxRange(
                70..70,
            ),
            ArenaIdxRange(
                70..71,
            ),
            ArenaIdxRange(
                71..71,
            ),
            ArenaIdxRange(
                71..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..73,
            ),
            ArenaIdxRange(
                73..73,
            ),
            ArenaIdxRange(
                73..74,
            ),
            ArenaIdxRange(
                74..78,
            ),
            ArenaIdxRange(
                78..89,
            ),
        ],
    },
)