use word::RootIdentifier;

use super::*;

impl HasStaticTypeInfo for () {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl<'eval> AnyValue<'eval> for () {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::Void(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::Void.into()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }
}

impl HasStaticTypeInfo for i32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl<'eval> AnyValue<'eval> for i32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::I32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
    fn print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::I32.into()
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }
}

impl HasStaticTypeInfo for f32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl<'eval> AnyValue<'eval> for f32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::F32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::F32.into()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }
}

impl HasStaticTypeInfo for u32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl<'eval> AnyValue<'eval> for u32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::B32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#032b}", self)
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::B32.into()
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }
}

impl HasStaticTypeInfo for u64 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl<'eval> AnyValue<'eval> for u64 {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::B64(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#064b}", self)
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }
}

impl HasStaticTypeInfo for bool {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl<'eval> AnyValue<'eval> for bool {
    fn clone_into_box<'temp>(&self) -> Box<dyn AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::Bool(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn static_ty() -> EntityRoutePtr {
        RootIdentifier::Bool.into()
    }
    fn print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }
}
