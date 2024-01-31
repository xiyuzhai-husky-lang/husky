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
                        Module,
                    ),
                },
            ),
        ],
    },
)