ValkyrieJavelinPantheon {
    package_path: PackagePath {
        toolchain: Toolchain {
            data: ToolchainData::Local {
                library_path: VirtualPath {
                    _data: RelPathBuf(
                        "../../../library",
                    ),
                },
            },
        },
        name: `quick-sort`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: RelPathBuf(
                    "../../../examples/algorithms/quick-sort",
                ),
            },
        },
    },
    instantiation_map: {},
    new_valkyrie_javelins: [],
}