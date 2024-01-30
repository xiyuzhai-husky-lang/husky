use enum_index::bitset::EnumBitSet;
use enum_index_macros::IsEnumIndex;
use Animal::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
enum Animal {
    Cat,
    Dog,
}

fn set_eq(set: EnumBitSet<Animal>, animals: &[Animal]) {
    assert_eq!(&set.iter().collect::<Vec<_>>(), animals)
}

#[test]
fn enum_bit_set_insert_remove_toggle_works() {
    let mut set: EnumBitSet<Animal> = Default::default();
    set.insert(Cat);
    set_eq(set, &[Cat]);
    set.insert(Dog);
    set_eq(set, &[Cat, Dog]);
    set.remove(Dog);
    set_eq(set, &[Cat]);
    set.remove(Cat);
    set_eq(set, &[]);
    set.insert(Dog);
    set_eq(set, &[Dog]);
    set.insert(Cat);
    set_eq(set, &[Cat, Dog]);
    set.remove(Cat);
    set_eq(set, &[Dog]);
    set.remove(Dog);
    set_eq(set, &[]);
    set.remove(Dog);
    set_eq(set, &[]);
    set.toggle(Dog);
    set_eq(set, &[Dog]);
    set.toggle(Dog);
    set_eq(set, &[]);
    set.insert(Cat);
    set.insert(Dog);
    set.toggle(Cat);
    set_eq(set, &[Dog]);
}

#[test]
fn enum_bit_set_map_works() {
    let mut set: EnumBitSet<Animal> = Default::default();
    set.insert(Cat);
    set_eq(set, &[Cat]);
    set.insert(Dog);
    set_eq(set, &[Cat, Dog]);
    let mapped: Vec<_> = set.iter().map(|elem| elem).collect();
    assert_eq!(mapped.len(), 2)
}
