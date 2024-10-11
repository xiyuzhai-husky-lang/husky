use hashlink::LinkedHashMap;
use serde::{de::VariantAccess, Deserialize};
use std::fmt::{self, format};
use yaml_rust2::Yaml;

pub struct AnyYaml {
    inner: Yaml,
}
impl AnyYaml {
    pub(crate) fn to_inner(self) -> Yaml {
        self.inner
    }
}

impl std::ops::Deref for AnyYaml {
    type Target = Yaml;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'de> Deserialize<'de> for AnyYaml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyYamlVisitor)
    }
}

pub struct AnyYamlVisitor;

impl<'a> serde::de::Visitor<'a> for AnyYamlVisitor {
    type Value = AnyYaml;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid YAML value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Boolean(v),
        })
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v),
        })
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Integer(v as i64),
        })
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Real(v.to_string()),
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::Real(v.to_string()),
        })
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::String(v.to_string()),
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml {
            inner: Yaml::String(v.to_owned()),
        })
    }

    fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Bytes(v),
            &self,
        ))
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(&v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml { inner: Yaml::Null })
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AnyYaml { inner: Yaml::Null })
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'a>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element::<AnyYaml>()? {
            vec.push(elem.inner);
        }
        Ok(AnyYaml {
            inner: Yaml::Array(vec),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'a>,
    {
        let mut hash = LinkedHashMap::new();
        while let Some((key, value)) = map.next_entry::<AnyYaml, AnyYaml>()? {
            hash.insert(key.inner, value.inner);
        }
        Ok(AnyYaml {
            inner: Yaml::Hash(hash),
        })
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'a>,
    {
        let (variant, variant_visitor) = data.variant()?;
        let value: AnyYaml = variant_visitor.newtype_variant()?;
        let mut hash = LinkedHashMap::new();
        hash.insert(Yaml::String(variant), value.inner);
        Ok(AnyYaml {
            inner: Yaml::Hash(hash),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust2::Yaml;

    #[test]
    fn test_any_yaml_hash_deserialization() {
        let yaml_str = r#"
        a: 1
        b: 2
        c: 3
        "#;

        let ordered: AnyYaml = serde_yml::from_str(yaml_str).unwrap();

        if let Yaml::Hash(hash) = &ordered.inner {
            let keys: Vec<_> = hash.keys().map(|k| k.as_str().unwrap()).collect();
            assert_eq!(keys, vec!["a", "b", "c"]);
        } else {
            panic!("Expected a Hash");
        }
    }

    #[test]
    fn test_any_yaml_array_deserialization() {
        let yaml_str = r#"
        - apple
        - banana
        - cherry
        "#;

        let ordered: AnyYaml = serde_yml::from_str(yaml_str).unwrap();

        if let Yaml::Array(array) = &ordered.inner {
            let items: Vec<_> = array.iter().map(|i| i.as_str().unwrap()).collect();
            assert_eq!(items, vec!["apple", "banana", "cherry"]);
        } else {
            panic!("Expected an Array");
        }
    }

    #[test]
    fn test_any_yaml_nested_structure() {
        let yaml_str = r#"
        person:
          name: John Doe
          age: 30
          hobbies:
            - reading
            - cycling
        "#;

        let ordered: AnyYaml = serde_yml::from_str(yaml_str).unwrap();

        if let Yaml::Hash(hash) = &ordered.inner {
            if let Yaml::Hash(person) = &hash[&Yaml::String("person".to_string())] {
                assert_eq!(
                    person[&Yaml::String("name".to_string())],
                    Yaml::String("John Doe".to_string())
                );
                assert_eq!(person[&Yaml::String("age".to_string())], Yaml::Integer(30));
                if let Yaml::Array(hobbies) = &person[&Yaml::String("hobbies".to_string())] {
                    assert_eq!(hobbies[0], Yaml::String("reading".to_string()));
                    assert_eq!(hobbies[1], Yaml::String("cycling".to_string()));
                } else {
                    panic!("Expected hobbies to be an Array");
                }
            } else {
                panic!("Expected person to be a Hash");
            }
        } else {
            panic!("Expected a Hash");
        }
    }

    #[test]
    fn test_any_yaml_mixed_types() {
        let yaml_str = r#"
        string: Hello, World!
        integer: 42
        float: 3.14
        boolean: true
        null_value: null
        "#;

        let ordered: AnyYaml = serde_yml::from_str(yaml_str).unwrap();

        if let Yaml::Hash(hash) = &ordered.inner {
            assert_eq!(
                hash[&Yaml::String("string".to_string())],
                Yaml::String("Hello, World!".to_string())
            );
            assert_eq!(
                hash[&Yaml::String("integer".to_string())],
                Yaml::Integer(42)
            );
            assert_eq!(
                hash[&Yaml::String("float".to_string())],
                Yaml::Real("3.14".to_string())
            );
            assert_eq!(
                hash[&Yaml::String("boolean".to_string())],
                Yaml::Boolean(true)
            );
            assert_eq!(hash[&Yaml::String("null_value".to_string())], Yaml::Null);
        } else {
            panic!("Expected a Hash");
        }
    }

    #[test]
    fn test_any_yaml_complex_structure() {
        let yaml_str = r#"
        company:
          name: TechCorp
          founded: 2000
          employees:
            - name: Alice
              role: Developer
              skills:
                - Rust
                - Python
            - name: Bob
              role: Designer
              skills:
                - UI/UX
                - Photoshop
        products:
          - name: SuperApp
            version: 1.0.0
            platforms:
              - Web
              - Mobile
        "#;

        let ordered: AnyYaml = serde_yml::from_str(yaml_str).unwrap();

        if let Yaml::Hash(hash) = &ordered.inner {
            // Check company details
            if let Yaml::Hash(company) = &hash[&Yaml::String("company".to_string())] {
                assert_eq!(
                    company[&Yaml::String("name".to_string())],
                    Yaml::String("TechCorp".to_string())
                );
                assert_eq!(
                    company[&Yaml::String("founded".to_string())],
                    Yaml::Integer(2000)
                );

                // Check employees
                if let Yaml::Array(employees) = &company[&Yaml::String("employees".to_string())] {
                    assert_eq!(employees.len(), 2);
                    if let Yaml::Hash(alice) = &employees[0] {
                        assert_eq!(
                            alice[&Yaml::String("name".to_string())],
                            Yaml::String("Alice".to_string())
                        );
                        assert_eq!(
                            alice[&Yaml::String("role".to_string())],
                            Yaml::String("Developer".to_string())
                        );
                    }
                }
            }

            // Check products
            if let Yaml::Array(products) = &hash[&Yaml::String("products".to_string())] {
                if let Yaml::Hash(super_app) = &products[0] {
                    assert_eq!(
                        super_app[&Yaml::String("name".to_string())],
                        Yaml::String("SuperApp".to_string())
                    );
                    assert_eq!(
                        super_app[&Yaml::String("version".to_string())],
                        Yaml::String("1.0.0".to_string())
                    );
                }
            }
        } else {
            panic!("Expected a Hash");
        }
    }
}
