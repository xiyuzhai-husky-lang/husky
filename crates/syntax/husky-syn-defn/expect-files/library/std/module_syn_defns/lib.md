Ok(
    [
        SynDefn::Submodule(
            SubmoduleSynDefn {
                decl: SubmoduleSynDecl {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `std::prelude`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ),
        SynDefn::Submodule(
            SubmoduleSynDefn {
                decl: SubmoduleSynDecl {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `std::logic`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ),
        SynDefn::Submodule(
            SubmoduleSynDefn {
                decl: SubmoduleSynDecl {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `std::ops`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ),
    ],
)