#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Pure,
    Share,
    Take,
    BorrowMut,
    TakeMut,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemberContract {
    Own,
    Ref,
}

impl MemberContract {
    pub fn constructor_input(&self) -> InputContract {
        match self {
            MemberContract::Own => InputContract::Take,
            MemberContract::Ref => InputContract::Share,
        }
    }
}
