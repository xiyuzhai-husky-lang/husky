Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ast_idx: 74,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    38,
                                                ),
                                                opd: 0,
                                            },
                                            Err(
                                                NoLeftOperandForBinaryOperator {
                                                    binary_token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                            ),
                                            Err(
                                                NoRightOperandForBinaryOperator {
                                                    lopd: 2,
                                                    punctuation: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    punctuation_token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                            ),
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    43,
                                                ),
                                                items: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                ),
                                            },
                                            Application {
                                                function: 4,
                                                argument: 5,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 181,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 30,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    46,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 279,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 29,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 1,
                                        },
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 6,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 285,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                    ty: 6,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    48,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                            ast_idx: 76,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    520,
                                                ),
                                                opd: 0,
                                            },
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    524,
                                                ),
                                                items: ArenaIdxRange(
                                                    2..2,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    525,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Application {
                                                function: 2,
                                                argument: 3,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    521,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 181,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 30,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    526,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 287,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 96,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            518,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 96,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 96,
                                                            },
                                                        ),
                                                    ),
                                                    access_start: TokenIdx(
                                                        519,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 4,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        517,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                519,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        522,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        523,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 4,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        527,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 75,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    },
                                },
                                ast_idx: 75,
                                body: ArenaIdxRange(
                                    39..53,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 31,
                                        },
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        51,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 31,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    50,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 287,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 256,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 39,
                                    ident: `norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 39,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 256,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 39,
                                                    ident: `norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            54,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            56,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `rel_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 324,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `rel_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 40,
                                    ident: `rel_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 40,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 324,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `rel_norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 40,
                                                    ident: `rel_norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            62,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            64,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `hausdorff_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 323,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `hausdorff_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 41,
                                    ident: `hausdorff_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 41,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 323,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `hausdorff_norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 41,
                                                    ident: `hausdorff_norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            80,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            82,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `angle_change`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 251,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `angle_change`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 42,
                                    ident: `angle_change`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 42,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 251,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `angle_change`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 42,
                                                    ident: `angle_change`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            167,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            169,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `bounding_box`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 183,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 43,
                                    ident: `bounding_box`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 43,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 183,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `bounding_box`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 43,
                                                    ident: `bounding_box`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        239,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 184,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            238,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            240,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `relative_bounding_box`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 193,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 44,
                                    ident: `relative_bounding_box`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 44,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 193,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `relative_bounding_box`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 44,
                                                    ident: `relative_bounding_box`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        368,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 194,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            367,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            369,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 100,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `line_segment`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 45,
                                    ident: `line_segment`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `line_segment`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 45,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 100,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `line_segment`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 45,
                                                    ident: `line_segment`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 284,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 33,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            384,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            385,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            386,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            388,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 197,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 46,
                                    ident: `start`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `start`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 46,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 197,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `start`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 46,
                                                    ident: `start`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        424,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            421,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            422,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            423,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            425,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 198,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 47,
                                    ident: `end`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `end`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 47,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 198,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `end`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 47,
                                                    ident: `end`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        444,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            441,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            442,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            443,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            445,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 196,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `displacement`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 48,
                                    ident: `displacement`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `displacement`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 48,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 196,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `displacement`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 48,
                                                    ident: `displacement`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        464,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            461,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            462,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            463,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            465,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 329,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start_tangent`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 49,
                                    ident: `start_tangent`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `start_tangent`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 49,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 329,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `start_tangent`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 49,
                                                    ident: `start_tangent`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        480,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            477,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            478,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            479,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            481,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 37,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 31,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 330,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end_tangent`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        body: ArenaIdxRange(
                                            39..53,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 31,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 50,
                                    ident: `end_tangent`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ident: `end_tangent`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 50,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 75,
                                                                body: ArenaIdxRange(
                                                                    39..53,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 31,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 287,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 31,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    stmt_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_region: PatternExprRegion {
                                                        pattern_expr_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                    },
                                                    symbol_region: SymbolRegion {
                                                        inherited_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: False,
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 330,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ident: `end_tangent`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 75,
                                                        body: ArenaIdxRange(
                                                            39..53,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 31,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 50,
                                                    ident: `end_tangent`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        498,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ParameterDeclList {
                                    lpar: LeftParenthesisToken {
                                        token_idx: TokenIdx(
                                            495,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            496,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            497,
                                        ),
                                    },
                                ),
                                output_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            499,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)