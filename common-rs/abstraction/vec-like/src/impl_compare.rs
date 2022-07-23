use crate::*;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};

impl<K, V> HuskyDisplay for VecMap<K, V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: VecMapEntry<K> + HuskyDisplay,
{
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        for entry in &self.entries {
            result.push_str("    ");
            entry.write_inherent(config.indented(), result);
            result.push('\n')
        }
    }
}
