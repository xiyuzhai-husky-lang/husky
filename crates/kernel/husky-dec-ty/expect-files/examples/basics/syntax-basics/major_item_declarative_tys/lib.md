[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
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
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
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
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
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
        ),
        Ok(
            DeclarativeTerm(`core::basic::Module`),
        ),
    ),
]