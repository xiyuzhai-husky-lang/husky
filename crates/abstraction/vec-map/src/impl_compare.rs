use crate::*;
use test_utils::TestComparable;

impl<K, V> TestComparable for VecMap<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K> + TestComparable,
{
    fn write_inherent(&self, result: &mut String) {
        for entry in &self.entries {
            entry.write_inherent(result);
            result.push_str("    ");
            result.push('\n')
        }
        println!("{}", &result);
        todo!()
    }
}
