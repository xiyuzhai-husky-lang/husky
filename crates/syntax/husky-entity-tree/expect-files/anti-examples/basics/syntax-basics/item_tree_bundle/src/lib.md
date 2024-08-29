```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`syntax_basics`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::ast),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics`),
                                ),
                                ast_idx: 0,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::ast),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::uses),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics`),
                                ),
                                ast_idx: 1,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::uses),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics`),
                                ),
                                ast_idx: 2,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `defn`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::expr),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics`),
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `expr`,
                                    token_idx: TokenIdx(
                                        8,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::expr),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `expr`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::ast),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::uses),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `defn`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::defn),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `expr`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::expr),
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
            module_path: ModulePath(`syntax_basics::ast`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::ast::submodule_name),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::ast`),
                                ),
                                ast_idx: 0,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::ast::submodule_name),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `submodule_name`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::ast`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `submodule_name`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::ast`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::ast::submodule_name),
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
            module_path: ModulePath(`syntax_basics::ast::submodule_name`),
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
            module_path: ModulePath(`syntax_basics::uses`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`syntax_basics::ast),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`syntax_basics::ast`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::uses`),
                                ),
                                ast_idx: 0,
                                use_expr_idx: 0,
                            },
                        ),
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`syntax_basics::uses),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`syntax_basics::uses`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::uses`),
                                ),
                                ast_idx: 0,
                                use_expr_idx: 0,
                            },
                        ),
                    },
                    EntitySymbolEntry {
                        ident: `defn`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`syntax_basics::defn),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`syntax_basics::defn`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::uses`),
                                ),
                                ast_idx: 0,
                                use_expr_idx: 0,
                            },
                        ),
                    },
                    EntitySymbolEntry {
                        ident: `expr`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
                        ),
                        symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Submodule {
                                    submodule_item_path: SubmoduleItemPath(`syntax_basics::expr),
                                },
                                path: PrincipalEntityPath::Module(
                                    ModulePath(`syntax_basics::expr`),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::uses`),
                                ),
                                ast_idx: 0,
                                use_expr_idx: 0,
                            },
                        ),
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [
                    OnceUseRule {
                        ast_idx: 0,
                        use_expr_idx: 1,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
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
                                0..1,
                            ),
                        },
                        parent: None,
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::CrateRoot {
                                    root_module_path: ModulePath(`syntax_basics`),
                                },
                            ),
                        },
                    },
                ],
            ),
            use_all_rules: UseAllRules(
                [
                    UseAllRule {
                        parent_module_path: ModulePath(`syntax_basics`),
                        is_same_crate: true,
                        ast_idx: 0,
                        use_expr_idx: 0,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::uses`),
                        ),
                        progress: Ok(
                            4,
                        ),
                    },
                ],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: ModulePath(`syntax_basics::defn`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::defn`),
                                ),
                                ast_idx: 0,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `major_item`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `major_item`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item),
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
            module_path: ModulePath(`syntax_basics::defn::major_item`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::defn::major_item`),
                                ),
                                ast_idx: 0,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ty`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ty`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty),
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
            module_path: ModulePath(`syntax_basics::defn::major_item::ty`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty::enum_ty),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::defn::major_item::ty`),
                                ),
                                ast_idx: 0,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty::enum_ty),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `enum_ty`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item::ty`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `enum_ty`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item::ty`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_basics::defn::major_item::ty::enum_ty),
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
            module_path: ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
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
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
                                ),
                                ast_idx: 5,
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
                                                0..5,
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
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
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
                            ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `A`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::defn::major_item::ty::enum_ty`),
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
        EntityTreeSheet {
            module_path: ModulePath(`syntax_basics::expr`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`syntax_basics::expr::nested`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::expr`),
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `nested`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`syntax_basics::expr::nested`, `Ritchie(
                                        Fn,
                                    )`),
                                    body: Some(
                                        FormBody {
                                            ast_idx_range: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`syntax_basics::expr::nested`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        ident: `nested`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::expr`),
                                ),
                                ast_idx: 4,
                                ident_token: IdentToken {
                                    ident: `closure_inline`,
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                        Fn,
                                    )`),
                                    body: Some(
                                        FormBody {
                                            ast_idx_range: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        ident: `closure_inline`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_basics::expr`),
                                ),
                                ast_idx: 5,
                                ident_token: IdentToken {
                                    ident: `closure_nested`,
                                    token_idx: TokenIdx(
                                        29,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                        Fn,
                                    )`),
                                    body: Some(
                                        FormBody {
                                            ast_idx_range: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        ident: `closure_nested`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `nested`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`syntax_basics::expr::nested`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `closure_inline`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`syntax_basics::expr::closure_inline`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `closure_nested`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_basics::expr`),
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`syntax_basics::expr::closure_nested`, `Ritchie(
                                    Fn,
                                )`),
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
```