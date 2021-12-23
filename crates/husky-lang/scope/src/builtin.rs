#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinIdentifier3 {
    Std,
    Core,
    Debug,
    I32,
    F32,
    Vec,
    Array(usize),
    Tuple,
    Fp,
    Fn,
    FnMut,
    FnOnce,
}
