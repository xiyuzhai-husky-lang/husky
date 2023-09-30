use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyCallTrace {}
