Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ast_idx: 22,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    12,
                                                ),
                                                opd: 0,
                                            },
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    9,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    10,
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    11,
                                                ),
                                                opd: 1,
                                            },
                                            Application {
                                                function: 2,
                                                argument: 3,
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    17,
                                                ),
                                                items: ArenaIdxRange(
                                                    5..5,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    19,
                                                ),
                                                opd: 5,
                                            },
                                            Application {
                                                function: 6,
                                                argument: 7,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    13,
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
                                            Root {
                                                token_idx: TokenIdx(
                                                    20,
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
                                            kind: FieldType,
                                            expr: 4,
                                        },
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 8,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 131,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    ty: 4,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 347,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                    ty: 8,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        14,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        21,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    22,
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
                            path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            ast_idx: 24,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    151,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    150,
                                                ),
                                                opd: 2,
                                            },
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    157,
                                                ),
                                                items: ArenaIdxRange(
                                                    4..4,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    158,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    161,
                                                ),
                                                opd: 5,
                                            },
                                            EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Bracketed {
                                                lpar_token_idx: TokenIdx(
                                                    160,
                                                ),
                                                item: 6,
                                                rpar_token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    165,
                                                ),
                                                opd: 7,
                                            },
                                            BinaryOpn {
                                                lopd: 8,
                                                opr: Curry,
                                                opr_token_idx: TokenIdx(
                                                    164,
                                                ),
                                                ropd: 9,
                                            },
                                            ApplicationOrFunctionCall {
                                                function: 4,
                                                lpar_token_idx: TokenIdx(
                                                    159,
                                                ),
                                                argument: 10,
                                                rpar_token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 3,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    153,
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
                                            Root {
                                                token_idx: TokenIdx(
                                                    162,
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
                                            Root {
                                                token_idx: TokenIdx(
                                                    166,
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
                                            Root {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 346,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 34,
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
                                                                    value: 286,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 350,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            155,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Parameter,
                                            Parameter,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 286,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 350,
                                                            },
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                Atom(
                                                    0,
                                                ),
                                                Atom(
                                                    1,
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
                                                                value: 286,
                                                            },
                                                        ),
                                                    ),
                                                    access_start: TokenIdx(
                                                        149,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 350,
                                                            },
                                                        ),
                                                    ),
                                                    access_start: TokenIdx(
                                                        156,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 3,
                                            },
                                            RegularParameter {
                                                pattern: 1,
                                                ty: 11,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 12,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        147,
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
                                                149,
                                            ),
                                        },
                                        ty: 3,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                156,
                                            ),
                                        },
                                        ty: 11,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            154,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        169,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 12,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        171,
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
                            ast_idx: 23,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::fermi`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    },
                                },
                                ast_idx: 23,
                                body: ArenaIdxRange(
                                    12..15,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 34,
                                        },
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        25,
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
                                                    module_path: `mnist_classifier::fermi`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 23,
                                                body: ArenaIdxRange(
                                                    12..15,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 34,
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
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 346,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 34,
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 34,
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
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 12,
                                    ident: `norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 12,
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 23,
                                                                body: ArenaIdxRange(
                                                                    12..15,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 34,
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 346,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 34,
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
                                                                    value: 34,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 34,
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
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ident: `norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::fermi`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 23,
                                                        body: ArenaIdxRange(
                                                            12..15,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 34,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 12,
                                                    ident: `norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::fermi`,
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
                                                        29,
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
                                            28,
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
                                            30,
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `rel_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 34,
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
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `rel_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 13,
                                    ident: `rel_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 13,
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 23,
                                                                body: ArenaIdxRange(
                                                                    12..15,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 34,
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 346,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 34,
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
                                                                    value: 34,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 34,
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
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ident: `rel_norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::fermi`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 23,
                                                        body: ArenaIdxRange(
                                                            12..15,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 34,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 13,
                                                    ident: `rel_norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::fermi`,
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
                                                        67,
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
                                            66,
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
                                            68,
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
                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ident: `angle_change_norm`,
                                        ty_item_kind: Memo,
                                    },
                                ),
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 34,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 348,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ident: `angle_change_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::fermi`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            },
                                        },
                                        ast_idx: 23,
                                        body: ArenaIdxRange(
                                            12..15,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 34,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 14,
                                    ident: `angle_change_norm`,
                                    associated_item_kind: TypeItem(
                                        Memo,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::fermi`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 14,
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
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 23,
                                                                body: ArenaIdxRange(
                                                                    12..15,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 34,
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
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 346,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 34,
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
                                                                    value: 34,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 34,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 348,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ident: `angle_change_norm`,
                                                            ty_item_kind: Memo,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::fermi`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 23,
                                                        body: ArenaIdxRange(
                                                            12..15,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 34,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 14,
                                                    ident: `angle_change_norm`,
                                                    associated_item_kind: TypeItem(
                                                        Memo,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::fermi`,
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
                                                        105,
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
                                            104,
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
                                            106,
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