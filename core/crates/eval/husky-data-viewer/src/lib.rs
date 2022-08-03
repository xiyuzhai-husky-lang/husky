mod query;

use husky_compile_time::HuskyCompileTime;
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_word::IdentPairDict;
pub use query::*;

use husky_vm_interface::{__Linkage, __LinkageFp, __Register, __RegistrableSafe};

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyDataViewer {
    Struct {
        fields: IdentPairDict<(__Linkage, EntityRoutePtr)>,
    },
    Vec {
        ilen: __LinkageFp,
        index: __Linkage,
    },
}

impl HuskyDataViewer {
    pub fn print<'eval>(&self, value: &__Register<'eval>) -> String {
        todo!()
    }

    pub fn serialize<'eval>(
        &self,
        comptime: &HuskyCompileTime,
        value: &__Register<'eval>,
    ) -> serde_json::Value {
        match self {
            HuskyDataViewer::Struct { fields } => serde_json::Value::Object(
                fields
                    .iter()
                    .map(|(ident, (linkage, ty))| todo!())
                    .collect(),
            ),
            HuskyDataViewer::Vec { ilen, index } => todo!(),
        }
    }

    pub fn member_eval_iter<'a, 'eval>(
        &'a self,
        value: &'a __Register<'eval>,
    ) -> impl Iterator<Item = (i32, __Register<'eval>)> + 'a {
        match self {
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
        }
    }
}
