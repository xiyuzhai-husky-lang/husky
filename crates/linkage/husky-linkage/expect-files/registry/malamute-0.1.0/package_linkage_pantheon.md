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
        name: `malamute`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: RelPathBuf(
                    "../../../registry/malamute-0.1.0",
                ),
            },
        },
    },
    instantiation_map: {},
    new_linkages: [],
}