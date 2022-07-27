use std::fmt::Display;
pub struct CPrimitiveTypeHeader<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct CPrimitiveTypeSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct CNonPrimitiveTypeSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CNonPrimitiveTypeSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
