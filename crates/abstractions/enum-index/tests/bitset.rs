use enum_index::bitset::EnumBitSet;
use enum_index_macros::IsEnumIndex;

#[test]
fn enum_bit_set_insert_remove_works() {
    use Animal::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
    enum Animal {
        Cat,
        Dog,
        Lizard,
    }

    fn t(set: EnumBitSet<Animal>, animals: &[Animal]) {
        assert_eq!(&set.to_vec(), animals)
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
}
