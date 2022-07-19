use std::borrow::Cow;

use husky_entity_route::EntityRoutePtr;
use print_utils::{msg_once, p};
use serde::Serialize;
use word::{CustomIdentifier, IdentPairDict};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualVec<'eval> {
    ty: EntityRoutePtr,
    data: Vec<MemberValue<'eval>>,
}

impl<'eval> VirtualVec<'eval> {
    pub fn new(ty: EntityRoutePtr, data: Vec<MemberValue<'eval>>) -> Self {
        Self { ty, data: vec![] }
    }
}

impl<'eval> std::ops::Deref for VirtualVec<'eval> {
    type Target = Vec<MemberValue<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for VirtualVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __HasStaticTypeInfo for VirtualVec<'eval> {
    type __StaticSelf = VirtualVec<'static>;

    fn __static_type_name() -> Cow<'static, str> {
        "[]Any".into()
    }
}

impl<'eval, 'eval0: 'eval> __AnyValue<'eval> for VirtualVec<'eval0> {
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(
            self.iter()
                .map(|elem| elem.any_ref().__to_json_value_dyn())
                .collect(),
        )
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        todo!()
    }

    fn __static_ty() -> EntityRoutePtr {
        panic!()
    }

    fn __ty(&self) -> EntityRoutePtr {
        self.ty
    }

    fn __print_short(&self) -> String {
        format!(
            "{{ len: {}, data: {} }}",
            self.len(),
            print_sequence(
                "{ ",
                self.iter(),
                &|value| format!("{}", value.any_ref().__print_short()),
                " }",
                20,
            )
        )
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Group(
            self.iter()
                .enumerate()
                .map(|(i, element)| visualize_element(i, element.any_ref().__short_dyn()))
                .collect::<__EvalResult<Vec<_>>>()?,
        )))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }
}
