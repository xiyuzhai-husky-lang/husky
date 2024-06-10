use shifted_unsigned_int::ShiftedU32;
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

pub struct Op<In, Out>(OpId, PhantomData<In>, PhantomData<Out>);

pub struct OpId(ShiftedU32);

pub struct OpStorage(Arc<Mutex<OpStorageInner>>);
pub struct OpStorageInner {
    data: HashMap<OpId, OpData>,
}

pub enum OpData {}
