use vec_like::ordered_vec_map::OrderedVecPairMap;
use yaml_rust2::yaml::Hash;
use yaml_rust2::Yaml;

/// `OrderedYaml` is a wrapper around `yaml_rust2::Yaml` that orders key-value pairs in YAML maps alphabetically by keys.
pub struct OrderedYaml {
    inner: Yaml,
}

impl OrderedYaml {
    pub fn new(yaml: &Yaml) -> Self {
        OrderedYaml {
            inner: Self::ordered_yaml(yaml),
        }
    }

    fn ordered_yaml(yaml: &Yaml) -> Yaml {
        match yaml {
            Yaml::Hash(hash) => Yaml::Hash(Self::ordered_hash(hash)),
            Yaml::Array(array) => Yaml::Array(array.into_iter().map(Self::ordered_yaml).collect()),
            scalar => scalar.clone(),
        }
    }

    fn ordered_hash(hash: &Hash) -> Hash {
        hash.iter()
            .map(|(key, value)| (Self::ordered_yaml(key), Self::ordered_yaml(value)))
            .collect::<OrderedVecPairMap<Yaml, Yaml>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
use yaml_rust2::YamlLoader;

#[test]
fn test_ordered_yaml_preserves_order() {
    let yaml_str = r#"
    c: 3
    a: 1
    b: 2
    "#;
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let ordered = OrderedYaml::new(&docs[0]);

    if let Yaml::Hash(hash) = ordered.inner {
        let keys: Vec<_> = hash.keys().map(|k| k.as_str().unwrap()).collect();
        assert_eq!(keys, vec!["a", "b", "c",]);
    }
}
