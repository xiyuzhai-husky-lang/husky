#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct ToolchainDate {
    year: Year,
    month: Month,
    day: Day,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Year(u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Month(u8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Day(u8);

impl ToolchainDate {
    pub fn new_ad_hoc() -> Self {
        Self {
            year: Year(2019),
            month: Month(8),
            day: Day(11),
        }
    }
}
