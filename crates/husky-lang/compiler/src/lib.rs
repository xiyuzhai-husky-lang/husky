use reserve::Reserve;
use scope::ScopePtr;
use semantics::{PackageQueryGroup, SemanticResult};

#[salsa::query_group(CompilerStorage)]
pub trait Compiler: PackageQueryGroup {
    fn compiled_rust_code(&self, scope: ScopePtr) -> SemanticResult<Option<Reserve<String>>>;
}

fn compiled_rust_code(
    this: &dyn Compiler,
    scope: ScopePtr,
) -> SemanticResult<Option<Reserve<String>>> {
    match scope.route {
        scope::ScopeRoute::Builtin { ident } => todo!(),
        scope::ScopeRoute::Package { main, ident } => todo!(),
        scope::ScopeRoute::ChildScope { parent, ident } => todo!(),
        scope::ScopeRoute::Implicit { main, ident } => todo!(),
    }
}
