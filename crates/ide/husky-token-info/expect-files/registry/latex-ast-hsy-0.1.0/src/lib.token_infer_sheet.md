```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Type(
                                TypeKind::Extern,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`latex_ast_hsy::AST`, `StaticVar`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::StaticVar,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`latex_ast_hsy::AST`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
        ],
    },
)
```