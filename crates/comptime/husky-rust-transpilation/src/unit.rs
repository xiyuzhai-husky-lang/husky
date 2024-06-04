use husky_vfs::path::crate_path::CratePath;

pub enum TranspilationUnit {
    Essential(EssentialTranspilationUnit),
    Complementary(ComplementaryTranspilationUnit),
}

pub struct EssentialTranspilationUnit {
    crate_path: CratePath,
}

pub struct ComplementaryTranspilationUnit {
    crate_path: CratePath,
}
