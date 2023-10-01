Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `std::prelude`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `std::logic`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::Submodule(
                            SubmoduleSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: SubmodulePath(
                                        `std::ops`,
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        Module,
                    ),
                },
            ),
        ],
    },
)