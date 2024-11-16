use crate::LnItemPath;

#[derive(Debug, PartialEq, Eq)]
pub struct LnItemPathMenu {
    pub nat: LnItemPath,
    pub rat: LnItemPath,
    pub int: LnItemPath,
    pub real: LnItemPath,
    pub complex: LnItemPath,
    pub ring_add: LnItemPath,
    pub ring_mul: LnItemPath,
    pub ring_pos: LnItemPath,
    pub ring_neg: LnItemPath,
    pub le: LnItemPath,
    pub ge: LnItemPath,
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
            ring_mul: LnItemPath::RING_MUL,
            ring_pos: LnItemPath::RING_POS,
            ring_neg: LnItemPath::RING_NEG,
            le: LnItemPath::LE,
            ge: LnItemPath::GE,
            eq: LnItemPath::EQ,
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_item_path_menu(db: &::salsa::Db) -> LnItemPathMenu {
    LnItemPathMenu::new(db)
}
