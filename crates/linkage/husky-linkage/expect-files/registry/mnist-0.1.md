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
        name: `mnist`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: RelPathBuf(
                    "../../../registry/mnist-0.1.0",
                ),
            },
        },
    },
    instantiation_map: {},
}