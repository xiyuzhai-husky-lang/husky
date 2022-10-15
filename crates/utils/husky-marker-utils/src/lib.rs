use std::{cell::Cell, marker::PhantomData, sync::MutexGuard};

pub type PhantomUnsync = PhantomData<Cell<()>>;
pub type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
