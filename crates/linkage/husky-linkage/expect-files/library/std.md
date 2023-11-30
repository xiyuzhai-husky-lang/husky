LinkagePantheon {
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
        name: `std`,
        data: PackagePathSource::Library,
    },
    instantiation_map: {},
    new_linkages: [],
}