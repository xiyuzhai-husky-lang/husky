use crate::*;

struct Player(usize);

type PlayerArena = Arena<Player>;

#[test]
fn find_rev_indexed_works() {
    let mut arena = PlayerArena::default();
    arena.alloc_batch([Player(0), Player(0)]);
    assert_eq!(
        arena.find_rev_indexed(|_| true).map(|(i, _)| i.raw()),
        Some(1)
    );
}
