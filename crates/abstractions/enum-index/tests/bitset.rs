use enum_index::bitset::EnumBitSet;
use enum_index_macros::IsEnumIndex;

#[test]
fn enum_bit_set_insert_remove_toggle_works() {
    use Animal::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
    enum Animal {
        Cat,
        Dog,
        Lizard,
    }

    fn t(set: EnumBitSet<Animal>, animals: &[Animal]) {
        assert_eq!(&set.iter().collect::<Vec<_>>(), animals)
    }

    let mut set: EnumBitSet<Animal> = Default::default();
    set.insert(Cat);
    t(set, &[Cat]);
    set.insert(Dog);
    t(set, &[Cat, Dog]);
    set.remove(Dog);
    t(set, &[Cat]);
    set.remove(Cat);
    t(set, &[]);
    set.insert(Dog);
    t(set, &[Dog]);
    set.insert(Cat);
    t(set, &[Cat, Dog]);
    set.remove(Cat);
    t(set, &[Dog]);
    set.remove(Dog);
    t(set, &[]);
    set.remove(Dog);
    t(set, &[]);
    set.toggle(Dog);
    t(set, &[Dog]);
    set.toggle(Dog);
    t(set, &[]);
    set.insert(Cat);
    set.insert(Dog);
    set.toggle(Cat);
    t(set, &[Dog]);
}
