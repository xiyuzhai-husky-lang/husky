use crate::LnItemPath;

#[derive(Debug, PartialEq, Eq)]
pub struct LnItemPathMenu {
    pub nat: LnItemPath,
    pub rat: LnItemPath,
    pub int: LnItemPath,
    pub real: LnItemPath,
    pub complex: LnItemPath,
    pub ring_add: LnItemPath,
    pub eq: LnItemPath,
}

impl LnItemPathMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        Self {
            nat: LnItemPath::NAT,
            rat: LnItemPath::RAT,
            int: LnItemPath::INT,
            real: LnItemPath::REAL,
            complex: LnItemPath::COMPLEX,
            ring_add: LnItemPath::RING_ADD,
            eq: LnItemPath::EQ,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_item_path_menu(db: &::salsa::Db) -> LnItemPathMenu {
    LnItemPathMenu::new(db)
}
