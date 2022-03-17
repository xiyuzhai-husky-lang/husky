#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Pure,
    Share,
    Take,
    BorrowMut,
    TakeMut,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MembVarContract {
    Own,
    Ref,
}

impl MembVarContract {
    pub fn constructor_input(&self) -> InputContract {
        match self {
            MembVarContract::Own => InputContract::Take,
            MembVarContract::Ref => InputContract::Share,
        }
    }
}
