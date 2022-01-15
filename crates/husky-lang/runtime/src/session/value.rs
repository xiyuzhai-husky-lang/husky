use common::*;

use crate::{any::Any, *};

pub enum SessionValue {
    Owned(Box<dyn Any>),
    Shared(&'static dyn Any),
}
