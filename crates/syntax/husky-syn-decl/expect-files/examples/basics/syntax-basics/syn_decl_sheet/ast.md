SynDeclSheet {
    decls: [
        (
            ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
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
            ),
            SynDecl::Submodule(
                SubmoduleSynDecl {
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
                },
            ),
        ),
    ],
}