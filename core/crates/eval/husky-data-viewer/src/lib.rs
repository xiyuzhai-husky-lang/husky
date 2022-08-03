mod query;

use husky_compile_time::HuskyCompileTime;
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_word::{IdentPairDict, RootIdentifier};
pub use query::*;

use husky_vm_interface::{__Linkage, __LinkageFp, __Register, __RegistrableSafe};

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyDataViewer {
    Primitive {
        ty: RootIdentifier,
    },
    Struct {
        fields: IdentPairDict<(__Linkage, EntityRoutePtr)>,
    },
    Vec {
        ilen: __LinkageFp,
        index: __Linkage,
    },
    CyclicSlice {
        start: __LinkageFp,
        end: __LinkageFp,
        index: __Linkage,
    },
}

impl HuskyDataViewer {
    pub fn print<'eval>(&self, value: &__Register<'eval>) -> String {
        todo!()
    }

    pub fn serialize<'eval>(
        &self,
        db: &dyn HuskyDataViewerQueryGroup,
        value: &__Register<'eval>,
    ) -> serde_json::Value {
        use serde::ser::Serialize;
        match self {
            HuskyDataViewer::Primitive { ty } => match ty {
                RootIdentifier::Void => todo!(),
                RootIdentifier::I32 => todo!(),
                RootIdentifier::I64 => todo!(),
                RootIdentifier::F32 => serde_json::to_value(value.downcast_f32()).unwrap(),
                RootIdentifier::F64 => todo!(),
                RootIdentifier::B32 => todo!(),
                RootIdentifier::B64 => todo!(),
                RootIdentifier::Bool => todo!(),
                _ => panic!(),
            },
            HuskyDataViewer::Struct { fields } => serde_json::Value::Object(
                fields
                    .iter()
                    .map(|(ident, (linkage, field_ty))| {
                        let value = linkage
                            .bind(Binding::TempRef)
                            .call(None, &mut vec![value.bind_temp_ref()]);
                        let field_data_viewer = db.ty_data_viewer(*field_ty);
                        let value: serde_json::Value = field_data_viewer.serialize(db, &value);
                        (ident.as_str().to_string(), value)
                    })
                    .collect(),
            ),
            HuskyDataViewer::Vec { ilen, index } => todo!(),
            HuskyDataViewer::CyclicSlice { start, end, index } => todo!(),
        }
    }

    pub fn member_eval_iter<'a, 'eval>(
        &'a self,
        value: &'a __Register<'eval>,
    ) -> impl Iterator<Item = (i32, __Register<'eval>)> + 'a {
        match self {
            HuskyDataViewer::Primitive { ty } => todo!(),
            HuskyDataViewer::Struct { fields } => todo!(),
            HuskyDataViewer::Vec { ilen, index } => {
                let ilen = ilen
                    .call(None, &mut vec![value.temp_bind_eval_ref()])
                    .downcast_i32();
                let index = index.bind(Binding::EvalRef);
                (0..ilen).into_iter().map(move |i| {
                    (
                        i,
                        index.call(None, &mut vec![value.temp_bind_eval_ref(), i.to_register()]),
                    )
                })
            }
            HuskyDataViewer::CyclicSlice { start, end, index } => todo!(),
        }
    }
}
