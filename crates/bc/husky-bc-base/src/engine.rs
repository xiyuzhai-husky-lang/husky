use crate::{db::BcDb, history::BcHistory, state::BcState};

pub struct BcEngine<'a> {
    db: &'a BcDb,
    state: BcState,
    history: BcHistory,
}