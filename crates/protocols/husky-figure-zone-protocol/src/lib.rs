pub mod chunk_base;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum FigureZone {
    Parade = 1,
    Roll,
}

impl FigureZone {
    pub fn len(self) -> usize {
        match self {
            FigureZone::Parade => 1,
            FigureZone::Roll => 1,
        }
    }
}
