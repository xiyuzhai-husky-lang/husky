use serde::Deserialize;
use std::fmt;
use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::yaml::Hash;
use yaml_rust2::Yaml;

use crate::any::AnyYamlVisitor;

/// `OrderedYaml` is a wrapper around `yaml_rust2::Yaml` that orders key-value pairs in YAML maps alphabetically by keys.
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct OrderedYaml {
    inner: Yaml,
}

impl<'de> Deserialize<'de> for OrderedYaml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let yaml = deserializer.deserialize_any(AnyYamlVisitor)?;
        Ok(OrderedYaml::new(yaml.to_inner()))
    }
}

impl OrderedYaml {
    pub fn new(mut yaml: Yaml) -> Self {
        Self::ordered_yaml(&mut yaml);
        OrderedYaml { inner: yaml }
    }

    fn ordered_yaml(yaml: &mut Yaml) {
        match yaml {
            Yaml::Hash(hash) => Self::ordered_hash(hash),
            Yaml::Array(array) => {
                for item in array.iter_mut() {
                    Self::ordered_yaml(item);
                }
            }
            _ => {}
        }
    }

    fn ordered_hash(hash: &mut Hash) {
        let mut ordered = OrderedVecPairMap::default();
        for (key, value) in hash.iter_mut() {
            let mut ordered_key = key.clone();
            let mut ordered_value = value.clone();
            Self::ordered_yaml(&mut ordered_key);
            Self::ordered_yaml(&mut ordered_value);
            ordered.insert((ordered_key, ordered_value));
        }
        *hash = ordered.into_iter().collect();
    }
}

impl fmt::Debug for OrderedYaml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderedYaml(\"",)?;
        yaml_rust2::YamlEmitter::new(f).dump(&self.inner).unwrap();
        write!(f, "\")",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_yaml_deserialization() {
        let yaml_str = r#"
        c: 3
        a: 1
        b: 2
        "#;

        let ordered: OrderedYaml = serde_yml::from_str(yaml_str).unwrap();
        // yaml_rust2::YamlLoader::load_from_str(yaml_str).unwrap();

        if let Yaml::Hash(hash) = &ordered.inner {
            let keys: Vec<_> = hash.keys().map(|k| k.as_str().unwrap()).collect();
            assert_eq!(keys, vec!["a", "b", "c"]);
        } else {
            panic!("Expected a Hash");
        }
    }
}
