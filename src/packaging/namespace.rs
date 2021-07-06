use linked_hash_map::LinkedHashMap;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Namespaces(LinkedHashMap<String, String>);

impl Namespaces {
    pub fn add_namespace<S1: Into<String>, S2: Into<String>>(&mut self, decl: S1, uri: S2) {
        self.0.insert(decl.into(), uri.into());
    }
}

impl<'de> Deserialize<'de> for Namespaces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(NamespaceVisitor)
    }
}

impl Serialize for Namespaces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

struct NamespaceVisitor;

impl<'de> Visitor<'de> for NamespaceVisitor {
    type Value = Namespaces;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("unexpected namespace attribute")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut ns = Namespaces::default();
        while let Some(key) = access.next_key()? {
            let key: String = key;
            match key {
                s if s.starts_with("xmlns") => {
                    let xmlns: String = access.next_value()?;
                    ns.add_namespace(s, xmlns);
                }

                s => {
                    log::debug!("unrecognized namespace: {}!", s);
                }
            }
        }
        Ok(ns)
    }
}
