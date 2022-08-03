use crate::*;
use entity_kind::TyKind;
use husky_compile_time::*;
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_word::RootIdentifier;
use std::sync::Arc;

#[salsa::query_group(HuskyDataViewerQueryGroupStorage)]
pub trait HuskyDataViewerQueryGroup: AskCompileTime {
    fn ty_data_viewer(&self, ty: EntityRoutePtr) -> Arc<HuskyDataViewer>;
}

fn ty_data_viewer(db: &dyn HuskyDataViewerQueryGroup, ty: EntityRoutePtr) -> Arc<HuskyDataViewer> {
    let ty_decl: Arc<TyDecl> = db.compile_time().ty_decl(ty).unwrap();
    Arc::new(match ty_decl.ty_kind {
        TyKind::Enum => todo!(),
        TyKind::Record => todo!(),
        TyKind::Struct => todo!(),
        TyKind::Primitive => todo!(),
        TyKind::Vec => HuskyDataViewer::Vec {
            len: todo!(),
            index: db
                .compile_time()
                .index_linkage(vec![ty, RootIdentifier::I32.into()])
                .bind(Binding::TempRef),
        },
        TyKind::Slice => todo!(),
        TyKind::CyclicSlice => todo!(),
        TyKind::Array => todo!(),
        TyKind::Tuple => todo!(),
        TyKind::Mor => todo!(),
        TyKind::Fp => todo!(),
        TyKind::AssociatedAny => todo!(),
        TyKind::ThisAny => todo!(),
        TyKind::SpatialPlaceholderAny => todo!(),
        TyKind::BoxAny => todo!(),
        TyKind::HigherKind => todo!(),
        TyKind::Ref => todo!(),
        TyKind::Option => todo!(),
    })
}
