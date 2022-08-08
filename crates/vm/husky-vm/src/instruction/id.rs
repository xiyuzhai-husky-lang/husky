use std::{
    ops::Deref,
    panic::RefUnwindSafe,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use husky_file::FilePtr;
use husky_text::TextRange;

static NEXT_VM_INSTRUCTION_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionId(pub(crate) usize);

pub trait InstructionSource: std::fmt::Debug + Send + Sync + RefUnwindSafe {
    fn instruction_id(&self) -> InstructionId;
    fn file(&self) -> FilePtr;
    fn text_range(&self) -> TextRange;
}

impl Default for InstructionId {
    fn default() -> Self {
        let raw = NEXT_VM_INSTRUCTION_ID.fetch_add(1, Ordering::Relaxed);
        Self(raw)
    }
}
