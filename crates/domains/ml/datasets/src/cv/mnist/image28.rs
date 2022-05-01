use std::{any::TypeId, sync::Arc};

use visual_syntax::VisualProps;
use vm::{AnyValue, AnyValueDyn, StaticTypeId};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28 {
    padded_rows: [u32; 30],
}

impl std::fmt::Debug for BinaryImage28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BinaryImage {{ padded_rows: [{:?}] }}",
            self.padded_rows
        ))
    }
}

impl BinaryImage28 {
    pub fn read(content: &[u8]) -> Self {
        assert_eq!(content.len(), 28 * 4);
        let mut padded_rows = [0; 30];
        for i in 0..28 {
            let mut row = 0u32;
            for k in 0..4 {
                row |= (content[i * 4 + k] as u32) << (3 - k) * 8;
            }
            padded_rows[i + 1] = row;
        }
        Self { padded_rows }
    }

    pub fn visualize<'eval>(value: &dyn AnyValueDyn<'eval>) -> VisualProps {
        let value: &BinaryImage28 = value.downcast_ref();
        VisualProps::BinaryImage28 {
            padded_rows: value.padded_rows.clone(),
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<u32> {
        self.padded_rows.get(index).map(|x| *x)
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.padded_rows.get_mut(index)
    }
}

impl<'eval> AnyValue<'eval> for BinaryImage28 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(self.clone())
    }

    fn print_short(&self) -> String {
        "BinaryImage28 { ... }".into()
    }
}
