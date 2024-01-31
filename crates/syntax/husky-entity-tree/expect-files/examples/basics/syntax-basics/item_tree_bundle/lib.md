EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: `syntax_basics`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::ast`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `ast`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::ast`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::uses`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `uses`,
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::uses`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::defn`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics`,
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `defn`,
                                    token_idx: TokenIdx(
                                        6,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::defn`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `defn`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::ast`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::uses`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `defn`,
                        visibility: Scope::PubUnder(
                            `syntax_basics`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::defn`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::ast`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::ast::submodule_name`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::ast`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `submodule_name`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::ast::submodule_name`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `submodule_name`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::ast`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `submodule_name`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::ast`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::ast::submodule_name`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::ast::submodule_name`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::uses`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(
                                        ItemPathId {
                                            data: ItemPathData::SubmoduleItem(
                                                SubmoduleItemPathData {
                                                    submodule_path: SubmodulePath(
                                                        `syntax_basics::ast`,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                },
                                path: PrincipalEntityPath::Module(
                                    `syntax_basics::ast`,
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::uses`,
                                ),
                                ast_idx: 1,
                                use_expr_idx: 1,
                            },
                        ),
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(
                                        ItemPathId {
                                            data: ItemPathData::SubmoduleItem(
                                                SubmoduleItemPathData {
                                                    submodule_path: SubmodulePath(
                                                        `syntax_basics::uses`,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                },
                                path: PrincipalEntityPath::Module(
                                    `syntax_basics::uses`,
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::uses`,
                                ),
                                ast_idx: 1,
                                use_expr_idx: 1,
                            },
                        ),
                    },
                    EntitySymbolEntry {
                        ident: `defn`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(
                                        ItemPathId {
                                            data: ItemPathData::SubmoduleItem(
                                                SubmoduleItemPathData {
                                                    submodule_path: SubmodulePath(
                                                        `syntax_basics::defn`,
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                },
                                path: PrincipalEntityPath::Module(
                                    `syntax_basics::defn`,
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::uses`,
                                ),
                                ast_idx: 1,
                                use_expr_idx: 1,
                            },
                        ),
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [
                    OnceUseRule {
                        ast_idx: 1,
                        use_expr_idx: 2,
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        variant: OnceUseRuleVariant::Parent {
                            parent_name_token: PathNameToken::CrateRoot(
                                CrateToken {
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            ),
                            children: ArenaIdxRange(
                                1..2,
                            ),
                        },
                        parent: None,
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::CrateRoot {
                                    root_module_path: `syntax_basics`,
                                },
                            ),
                        },
                    },
                ],
            ),
            use_all_rules: UseAllRules(
                [
                    UseAllRule {
                        parent_module_path: `syntax_basics`,
                        is_same_crate: true,
                        ast_idx: 1,
                        use_expr_idx: 1,
                        visibility: Scope::PubUnder(
                            `syntax_basics::uses`,
                        ),
                        progress: Ok(
                            3,
                        ),
                    },
                ],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::defn`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::defn::major_item`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::defn`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `major_item`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::defn::major_item`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `major_item`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `major_item`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::defn::major_item`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::defn::major_item`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::defn::major_item::ty`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::defn::major_item`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `ty`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::defn::major_item::ty`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ty`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ty`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::defn::major_item::ty`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::defn::major_item::ty`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: SubmoduleItemPath(
                                                        ItemPathId {
                                                            data: ItemPathData::SubmoduleItem(
                                                                SubmoduleItemPathData {
                                                                    submodule_path: SubmodulePath(
                                                                        `syntax_basics::defn::major_item::ty::enum_ty`,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::defn::major_item::ty`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `enum_ty`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId {
                                                        data: ItemPathData::SubmoduleItem(
                                                            SubmoduleItemPathData {
                                                                submodule_path: SubmodulePath(
                                                                    `syntax_basics::defn::major_item::ty::enum_ty`,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `enum_ty`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item::ty`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `enum_ty`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item::ty`,
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `syntax_basics::defn::major_item::ty::enum_ty`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: `syntax_basics::defn::major_item::ty::enum_ty`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `syntax_basics::defn::major_item::ty::enum_ty`,
                                ),
                                ast_idx: 6,
                                ident_token: IdentToken {
                                    ident: `A`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `A`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item::ty::enum_ty`,
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `A`,
                        visibility: Scope::PubUnder(
                            `syntax_basics::defn::major_item::ty::enum_ty`,
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [],
    },
}