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
            DecTerm(`core::basic::Module`),
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
            DecTerm(`core::basic::Module`),
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
            DecTerm(`core::basic::Module`),
        ),
    ),
]