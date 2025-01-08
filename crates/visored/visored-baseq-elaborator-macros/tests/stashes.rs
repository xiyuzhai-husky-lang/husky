use visored_baseq_elaborator_macros::stashes;

#[stashes]
struct Stashes {
    stash1: Stash1,
    stash2: Stash2,
    stash3: Stash3,
}

struct Stash1;
struct Stash2;
struct Stash3;

#[derive(Clone, Copy)]
struct Record;

struct Entry;

impl Stash1 {
    fn add_hypothesis(&mut self, record: Record, entry: &Entry) {
        todo!()
    }
}

impl Stash2 {
    fn add_hypothesis(&mut self, record: Record, entry: &Entry) {
        todo!()
    }
}

impl Stash3 {
    fn add_hypothesis(&mut self, record: Record, entry: &Entry) {
        todo!()
    }
}

impl Stashes {
    pub fn add_hypothesis(&mut self, record: Record, entry: &Entry) {
        self._add_hypothesis(record, entry);
    }
}
