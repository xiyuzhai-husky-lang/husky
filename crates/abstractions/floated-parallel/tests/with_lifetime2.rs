use floated_parallel::{db::FloaterDb, floated, note};

#[floated]
pub struct List<'db> {
    pub data: ListData<'db>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ListData<'db> {
    Nil,
    Cons(i32, List<'db>),
}

impl<'db> std::fmt::Debug for List<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[note]
fn len<'db>(list: List<'db>, db: &'db FloaterDb) -> usize {
    match list.data() {
        ListData::Nil => 0,
        ListData::Cons(_, tail) => 1 + len(tail, db),
    }
}

#[test]
fn list_len_works() {
    let db = &FloaterDb::default();
    let list = List::new(ListData::Cons(1, List::new(ListData::Nil, db)), db);
    assert_eq!(len(list, db), 1);
}
