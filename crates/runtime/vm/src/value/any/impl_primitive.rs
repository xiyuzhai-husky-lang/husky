use super::*;

impl<'eval> AnyValue<'eval> for () {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::Void(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }
}

impl<'eval> AnyValue<'eval> for i32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::I32(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }
}

impl<'eval> AnyValue<'eval> for f32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "f32".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::F32(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }
}

impl<'eval> AnyValue<'eval> for u32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u32".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::B32(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
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
}

impl<'eval> AnyValue<'eval> for u64 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u64".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::B64(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
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
}

impl<'eval> AnyValue<'eval> for bool {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    fn clone_into_box<'vm>(&self) -> Box<dyn AnyValueDyn<'eval> + 'vm>
    where
        Self: 'vm,
    {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: TempValue) -> Self {
        match stack_value {
            TempValue::Copyable(CopyableValue::Bool(value)) => value,
            TempValue::EvalOwned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::to_value(self).unwrap()
    }
}
