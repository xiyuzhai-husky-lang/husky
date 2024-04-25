use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaceIdx(ShiftedU32);

impl std::fmt::Debug for PlaceIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlaceIdx({})", self.0.index())
    }
}

/// # getters

impl PlaceIdx {
    pub fn index(self) -> usize {
        self.0.index()
    }
}

impl PlaceRegistry {
    pub fn len(&self) -> usize {
        self.infos.len()
    }
}

impl std::ops::Index<PlaceIdx> for PlaceRegistry {
    type Output = PlaceInfo;

    fn index(&self, idx: PlaceIdx) -> &Self::Output {
        &self.infos[idx.0.index()]
    }
}

/// # actions

impl PlaceRegistry {
    pub fn issue_new(&mut self, info: PlaceInfo) -> PlaceIdx {
        let next = self.infos.len();
        self.infos.push(info);
        PlaceIdx(next.into())
    }
}
