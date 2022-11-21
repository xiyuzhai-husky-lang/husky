use crate::TermJar;

#[salsa::interned(jar = TermJar)]
pub struct TermCurryContext {
    function: (),
    idx: usize,
}
