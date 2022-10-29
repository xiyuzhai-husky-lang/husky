use idx_arena::IdxArena;

pub struct TrackableIdxArena<E> {
    inner: IdxArena<E>,
    state: TrackableIdxArenaState,
}

pub struct TrackableIdxArenaState {
    original_size: usize,
}
