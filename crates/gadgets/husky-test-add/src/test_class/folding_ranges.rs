use crate::*;

#[derive(Subcommand)]
pub(crate) enum FoldingRangesTestOrder {
    Misc,
}

impl FoldingRangesTestOrder {
    pub(crate) fn relative_path_str(&self) -> &'static str {
        match self {
            FoldingRangesTestOrder::Misc => "tests/folding-ranges/misc",
        }
    }
}
