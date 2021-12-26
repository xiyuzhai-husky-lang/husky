use crate::*;

pub type PureFp = fn(Vec<&dyn Any>) -> Result<Box<dyn Any>, RuntimeError>;

pub type PureFn = Box<dyn Fn(Vec<&dyn Any>) -> Result<Box<dyn Any>, RuntimeError>>;

pub type PureFnOnce = Box<dyn FnOnce(Vec<&dyn Any>) -> Result<Box<dyn Any>, RuntimeError>>;

pub type AnyFp<'a> = fn(
    Vec<Box<dyn Any>>,
    Vec<&dyn Any>,
    Vec<&'a dyn Any>,
    Vec<&'static dyn Any>,
) -> Result<&'a dyn Any, RuntimeError>;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub type MemberAccess = for<'a> fn(&'a dyn Any) -> RuntimeResult<&'a dyn Any>;

pub type MemberAccessMut = for<'a> fn(&'a dyn Any) -> RuntimeResult<&'a dyn Any>;

pub type ElementAccess = for<'a> fn(&'a mut dyn Any, i: usize) -> RuntimeResult<&'a mut dyn Any>;

pub type ElementAccessMut = for<'a> fn(&'a mut dyn Any, i: usize) -> RuntimeResult<&'a mut dyn Any>;

struct A {
    a: isize,
}

fn ma<'a>(val: &'a dyn Any) -> RuntimeResult<&'a dyn Any> {
    let a: &A = val.downcast_ref().ok_or(RuntimeError::DowncastError)?;
    Ok(&a.a)
}

#[test]
fn haha() {
    let opn: MemberAccess = ma;
}
