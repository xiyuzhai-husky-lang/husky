use crate::*;
use test_utils::{TestComparable, TestCompareConfig};

impl<K, V> TestComparable for VecMap<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K> + TestComparable,
{
    fn write_inherent(&self, config: TestCompareConfig, result: &mut String) {
        for entry in &self.entries {
            result.push_str("    ");
            entry.write_inherent(config.indented(), result);
            result.push('\n')
        }
    }
}
