#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Intact,
    Share,
    Own,
    MutShare,
    MutOwn,
}
