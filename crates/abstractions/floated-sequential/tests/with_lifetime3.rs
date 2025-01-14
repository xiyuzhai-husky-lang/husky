use floated_sequential::{db::FloaterDb, floated, note};

#[floated(constructor = pub new)]
pub struct List<'sess> {
    pub data: ListData<'sess>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ListData<'sess> {
    Nil,
    Cons(i32, List<'sess>),
}

impl<'sess> std::fmt::Debug for List<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[note]
fn len<'sess>(list: List<'sess>, db: &'sess FloaterDb) -> usize {
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
