use semantics::PackageQueryGroup;

#[salsa::query_group(CompilerStorage)]
pub trait Compiler: PackageQueryGroup {}
