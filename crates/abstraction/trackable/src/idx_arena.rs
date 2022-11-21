use idx_arena::Arena;

pub struct TrackableIdxArena<E> {
    inner: Arena<E>,
    state: TrackableIdxArenaState,
}

pub struct TrackableIdxArenaState {
    original_size: usize,
}
