#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Contract {
    Pure,
    Share,
    Take,
    BorrowMut,
    TakeMut,
}
