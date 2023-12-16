use super::*;
use crate::r#static::{Static, StaticDyn};

pub trait Snapshot: std::fmt::Debug + Clone + RefUnwindSafe + UnwindSafe + 'static {
    type Static: Static;
    type Stand: std::any::Any;

    /// this function gives back the value snapshoted,
    /// together with a stand so that the value is valid if the stand is not dropped.
    ///
    /// Returns None if Stand is trivial to save a call of `Box::new`.
    fn revive(&self) -> (Option<Self::Stand>, Self::Static);
}

pub enum Stand {
    Box(Box<dyn std::any::Any>),
    Arc(Arc<dyn SnapshotDyn>),
}

pub trait SnapshotDyn: std::fmt::Debug {
    /// returns a owned type and the stand it needed
    fn revive_dyn(&self) -> (Option<Stand>, Box<dyn StaticDyn>);
    fn revive_ref_dyn(self: Arc<Self>) -> (Stand, *const dyn StaticDyn);
    fn revive_mut_dyn(&self) -> (Stand, *mut dyn StaticDyn);
}

impl<T> SnapshotDyn for T
where
    T: Snapshot,
{
    fn revive_dyn(&self) -> (Option<Stand>, Box<dyn StaticDyn>) {
        let (stand, static_self) = self.revive();
        (
            stand.map(|stand| Stand::Box(Box::new(stand))),
            Box::new(static_self),
        )
    }

    fn revive_ref_dyn(self: Arc<Self>) -> (Stand, *const dyn StaticDyn) {
        let slf: *const <Self as Snapshot>::Static =
            unsafe { std::mem::transmute(&*self as *const Self) };
        (Stand::Arc(self), slf)
    }

    fn revive_mut_dyn(&self) -> (Stand, *mut dyn StaticDyn) {
        let mut slf = self.clone();
        let slf_mut: *mut <Self as Snapshot>::Static =
            unsafe { std::mem::transmute(&mut slf as *mut Self) };
        (Stand::Box(Box::new(slf)), slf_mut)
    }
}

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum ValueSnapshot {
    /// useful for snapshot caching on stack
    None,
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    Box(Arc<dyn SnapshotDyn>),
    Leash(&'static dyn StaticDyn),
    SizedRef(Arc<dyn SnapshotDyn>),
    SizedRefMut(Arc<dyn SnapshotDyn>),
    OptionBox(Option<Arc<dyn SnapshotDyn>>),
    OptionLeash(Option<&'static dyn StaticDyn>),
    OptionSizedRef(Option<Arc<dyn SnapshotDyn>>),
    OptionSizedRefMut(Option<Arc<dyn SnapshotDyn>>),
    Intrinsic(Arc<dyn SnapshotDyn>),
}

impl Value {
    pub unsafe fn snapshot(&self) -> ValueSnapshot {
        match self {
            Value::Moved => ValueSnapshot::Moved,
            Value::Invalid => ValueSnapshot::Invalid,
            Value::Unit(_) => ValueSnapshot::Unit(()),
            Value::Bool(val) => ValueSnapshot::Bool(*val),
            Value::Char(val) => ValueSnapshot::Char(*val),
            Value::I8(val) => ValueSnapshot::I8(*val),
            Value::I16(val) => ValueSnapshot::I16(*val),
            Value::I32(val) => ValueSnapshot::I32(*val),
            Value::I64(val) => ValueSnapshot::I64(*val),
            Value::I128(val) => ValueSnapshot::I128(*val),
            Value::ISize(val) => ValueSnapshot::ISize(*val),
            Value::U8(val) => ValueSnapshot::U8(*val),
            Value::U16(val) => ValueSnapshot::U16(*val),
            Value::U32(val) => ValueSnapshot::U32(*val),
            Value::U64(val) => ValueSnapshot::U64(*val),
            Value::U128(val) => ValueSnapshot::U128(*val),
            Value::USize(val) => ValueSnapshot::USize(*val),
            Value::R8(val) => ValueSnapshot::R8(*val),
            Value::R16(val) => ValueSnapshot::R16(*val),
            Value::R32(val) => ValueSnapshot::R32(*val),
            Value::R64(val) => ValueSnapshot::R64(*val),
            Value::R128(val) => ValueSnapshot::R128(*val),
            Value::RSize(val) => ValueSnapshot::RSize(*val),
            Value::F32(val) => ValueSnapshot::F32(*val),
            Value::F64(val) => ValueSnapshot::F64(*val),
            Value::StringLiteral(id) => ValueSnapshot::StringLiteral(*id),
            Value::Box(box_value) => ValueSnapshot::Box(box_value.snapshot()),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
            Value::Intrinsic(_) => todo!(),
        }
    }
}
