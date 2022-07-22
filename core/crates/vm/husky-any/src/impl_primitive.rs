use word::RootIdentifier;

use super::*;

impl __HasStaticTypeInfo for () {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl<'eval> __AnyValue<'eval> for () {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::Void(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::Void.into()
    }

    fn __print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        todo!()
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}

impl __HasStaticTypeInfo for i32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl<'eval> __AnyValue<'eval> for i32 {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::I32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
    fn __print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::I32.into()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Copyable(self.into())
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}

impl __HasStaticTypeInfo for f32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl<'eval> __AnyValue<'eval> for f32 {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::F32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::F32.into()
    }

    fn __print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Copyable(self.into())
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}

impl __HasStaticTypeInfo for u32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl<'eval> __AnyValue<'eval> for u32 {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::B32(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn __print_short(&self) -> String {
        format!("{:#032b}", self)
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::B32.into()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Copyable(self.into())
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}

impl __HasStaticTypeInfo for u64 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl<'eval> __AnyValue<'eval> for u64 {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::B64(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn __print_short(&self) -> String {
        format!("{:#064b}", self)
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Copyable(self.into())
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}

impl __HasStaticTypeInfo for bool {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl<'eval> __AnyValue<'eval> for bool {
    fn __clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn __take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn __from_stack(stack_value: __TempValue) -> Self {
        match stack_value {
            __TempValue::Copyable(CopyableValue::Bool(value)) => value,
            __TempValue::OwnedEval(boxed_value) => boxed_value.downcast_move().unwrap(),
            _ => panic!(),
        }
    }

    fn __clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn __to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        RootIdentifier::Bool.into()
    }
    fn __print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn __opt_visualize(
        &'eval self,
        visualize_element: &mut dyn FnMut(
            usize,
            &'eval dyn __AnyValueDyn<'eval>,
        ) -> __EvalResult<VisualData>,
    ) -> __EvalResult<Option<VisualData>> {
        Ok(Some(VisualData::Primitive { value: self.into() }))
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Copyable(self.into())
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        __TempValue::Copyable(self.into())
    }
}
