#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinScope {
    Std,
    Core,
    Debug,
    I32,
    F32,
    Vec,
    Array(usize),
    Tuple,
    Rp,
    Rt,
    RtMut,
    RtOnce,
}
