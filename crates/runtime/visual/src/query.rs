use crate::*;
#[salsa::query_group(VisualQueryGroupStorage)]
pub trait VisualQueryGroup: AskCompileTime {
    fn visualizer(&self, version: usize, ty: ScopePtr) -> Arc<RuntimeVisualizer>;
}

fn visualizer(db: &dyn VisualQueryGroup, version: usize, ty: ScopePtr) -> Arc<RuntimeVisualizer> {
    let scope_source = db.compile_time(version).scope_source(ty).unwrap();
    match scope_source {
        ScopeSource::Builtin(builtin_scope_data) => match builtin_scope_data.signature {
            BuiltinScopeSignature::Func(_) => todo!(),
            BuiltinScopeSignature::Ty { ref visualizer, .. } => Arc::new(visualizer.into()),
            BuiltinScopeSignature::Module => todo!(),
        },
        ScopeSource::WithinBuiltinModule => todo!(),
        ScopeSource::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        ScopeSource::Module { file } => todo!(),
        ScopeSource::Implicit { main, ident } => todo!(),
    }
}
