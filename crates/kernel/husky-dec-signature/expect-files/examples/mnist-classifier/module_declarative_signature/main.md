[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::connected_component`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::raw_contour`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::geom2d`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::line_segment_sketch`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::fermi`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::digits`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
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
                                `mnist_classifier::major`,
                            ),
                        },
                    ),
                },
            ),
        ),
        Ok(
            DecTemplate::Submodule,
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::main`, `Val`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm(`malamute::Class mnist::MnistLabel`),
                        },
                    ),
                ),
            ),
        ),
    ),
]