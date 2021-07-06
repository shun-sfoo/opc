use std::io::BufRead;

use linked_hash_map::LinkedHashMap;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;

pub const RELATIONSHIPS_FILE: &'static str = "_rels/.rels";
const XMLNS_ATTRIBUTE_NAME: &'static str = "xmlns";
const RELATIONSHIP_NAMESPACE_URI: &'static str =
    "http://schemas.openxmlformats.org/package/2006/relationships";
// const XMLNS_R_ATTRIBUTE_NAME: &'static str = "xmlns:r";
const RELATIONSHIP_TAG_NAME: &'static str = "Relationship";
const RELATIONSHIPS_TAG_NAME: &'static str = "Relationships";

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Relationships {
    relationships: LinkedHashMap<String, Relationship>,
}

#[derive(Debug, PartialEq, Default, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relationship {
    id: String,
    r#type: String,
    target: String,
}

impl Relationship {
    pub fn target(&self) -> &str {
        &self.target
    }
}

impl Relationships {
    pub fn parse_from_xml_reader<R: BufRead>(reader: R) -> Self {
        quick_xml::de::from_reader(reader).unwrap()
    }

    pub fn parse_from_xml_str(reader: &str) -> Self {
        quick_xml::de::from_str(reader).unwrap()
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships
            .insert(relationship.id.clone(), relationship);
    }

    pub fn is_empty(&self) -> bool {
        self.relationships.is_empty()
    }

    pub fn get_relationship_by_id(&self, id: &str) -> Option<&Relationship> {
        self.relationships.get(id)
    }
}

impl<'de> Deserialize<'de> for Relationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationshipsVisitor)
    }
}

struct RelationshipsVisitor;

impl<'de> Visitor<'de> for RelationshipsVisitor {
    type Value = Relationships;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("relationships deserializing error")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut types: Relationships = Relationships::default();
        while let Some(key) = access.next_key()? {
            let key: String = key;
            match &key {
                s if s == XMLNS_ATTRIBUTE_NAME => {
                    let _xmlns: String = access.next_value()?;
                }

                s if s == RELATIONSHIPS_TAG_NAME => {
                    unreachable!();
                }

                s if s == RELATIONSHIP_TAG_NAME => {
                    let v: Relationship = access.next_value()?;
                    types.add_relationship(v);
                }
                _ => {
                    println!("{}", key);
                    unreachable!("relationships unsupport!");
                }
            }
        }
        Ok(types)
    }
}
