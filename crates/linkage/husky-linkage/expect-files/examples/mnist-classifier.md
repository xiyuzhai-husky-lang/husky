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
        name: `mnist-classifier`,
        data: PackagePathSource::Local {
            path: VirtualPath {
                _data: RelPathBuf(
                    "../../../examples/mnist-classifier",
                ),
            },
        },
    },
    instantiation_map: {},
}