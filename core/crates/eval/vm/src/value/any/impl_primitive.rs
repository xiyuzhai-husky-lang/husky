use word::RootIdentifier;

use super::*;

impl HasStaticTypeInfo for () {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl<'eval> __AnyValue<'eval> for () {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::Void(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
}

impl HasStaticTypeInfo for i32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl<'eval> __AnyValue<'eval> for i32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::I32(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
    fn print_short(&self) -> String {
        format!("{:?}", self)
    }

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::I32.into()
    }
}

impl HasStaticTypeInfo for f32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl<'eval> __AnyValue<'eval> for f32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::F32(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::F32.into()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
}

impl HasStaticTypeInfo for u32 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl<'eval> __AnyValue<'eval> for u32 {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::B32(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#032b}", self)
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::B32.into()
    }
}

impl HasStaticTypeInfo for u64 {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl<'eval> __AnyValue<'eval> for u64 {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::B64(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#064b}", self)
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        todo!()
    }
}

impl HasStaticTypeInfo for bool {
    type StaticSelf = Self;
    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl<'eval> __AnyValue<'eval> for bool {
    fn clone_into_box<'temp>(&self) -> Box<dyn __AnyValueDyn<'eval> + 'temp>
    where
        Self: 'temp,
    {
        Box::new(*self)
    }

    fn take_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::Bool(value)) => value,
            TempValue::OwnedEval(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn __AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn ty(&self) -> EntityRoutePtr {
        RootIdentifier::Bool.into()
    }
    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
}
