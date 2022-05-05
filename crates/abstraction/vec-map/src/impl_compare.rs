use crate::*;
use test_utils::TestComparable;

impl<K, V> TestComparable for VecMap<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K> + TestComparable,
{
    fn write_inherent(&self, result: &mut String) {
        result.push_str("eager variable qualified types:\n\n");
        for entry in &self.entries {
            entry.write_inherent(result);
            result.push('\n')
        }
        println!("{}", &result);
        todo!()
    }
}
