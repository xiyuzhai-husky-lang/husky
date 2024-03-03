use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaceIdx(ShiftedU32);

impl std::fmt::Debug for PlaceIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlaceIdx({})", self.0.index())
    }
}

impl std::ops::Index<PlaceIdx> for PlaceRegistry {
    type Output = PlaceInfo;

    fn index(&self, idx: PlaceIdx) -> &Self::Output {
        &self.infos[idx.0.index()]
    }
}

impl PlaceRegistry {
    pub fn issue_new(&mut self, info: PlaceInfo) -> PlaceIdx {
        debug_assert_eq!(self.infos.len(), self.next.index());
        let next = self.next;
        self.infos.push(info);
        self.next += 1;
        PlaceIdx(next)
    }
}
