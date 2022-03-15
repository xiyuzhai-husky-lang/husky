#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Contract {
    PureInput,
    Share,
    Take,
    BorrowMut,
    TakeMut,
}
