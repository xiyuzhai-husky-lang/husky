#[derive(Debug, PartialEq, Eq)]
pub enum FieldSignature<T: Copy> {
    Regular(RegularFieldSignature<T>),
    Memoized(MemoizedFieldSignature<T>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularFieldSignature<T: Copy> {
    modifier: (),
    ty: T,
}

impl<T: Copy> RegularFieldSignature<T> {
    pub fn modifier(&self) -> () {
        self.modifier
    }

    pub fn ty(&self) -> T {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MemoizedFieldSignature<T: Copy> {
    modifier: (),
    ty: T,
}
