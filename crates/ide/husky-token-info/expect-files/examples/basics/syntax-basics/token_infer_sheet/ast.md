```rust
Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: SubmoduleItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        EntityKind::Module,
                    ),
                },
            ),
        ],
    },
)
```