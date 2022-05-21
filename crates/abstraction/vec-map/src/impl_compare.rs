use crate::*;
use test_utils::{TestDisplay, TestDisplayConfig};

impl<K, V> TestDisplay for VecMap<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: HasKey<K> + TestDisplay,
{
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        for entry in &self.entries {
            result.push_str("    ");
            entry.write_inherent(config.indented(), result);
            result.push('\n')
        }
    }
}
