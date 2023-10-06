use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceId(ShiftedU32);

impl TraceId {
    pub(crate) fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub(crate) fn index(self) -> usize {
        self.0.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceIdRange {
    start: TraceId,
    end: TraceId,
}

impl TraceIdRange {
    pub(crate) fn new(start: TraceId, end: TraceId) -> Self {
        Self { start, end }
    }

    #[cfg(feature = "mock")]
    pub(crate) fn new_mock(start: usize, end: usize) -> Self {
        Self {
            start: TraceId::from_index(start),
            end: TraceId::from_index(end),
        }
    }

    pub fn start(&self) -> TraceId {
        self.start
    }

    pub fn end(&self) -> TraceId {
        self.end
    }

    pub fn into_iter(self) -> impl Iterator<Item = TraceId> {
        (self.start.index()..self.end.index())
            .into_iter()
            .map(|index| TraceId::from_index(index))
    }
}
