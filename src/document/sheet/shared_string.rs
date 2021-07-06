use crate::packaging::element::OpenXmlDeserializeDefault;
use crate::packaging::namespace::Namespaces;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"), rename = "sst")]
pub struct SharedStringsPart {
    count: usize,
    unique_count: usize,
    #[serde(flatten)]
    namespaces: Namespaces,
    #[serde(rename = "si")]
    strings: Option<Vec<SharedString>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all(deserialize = "camelCase"), rename = "si")]
pub struct SharedString {
    t: Option<String>,
}

impl OpenXmlDeserializeDefault for SharedStringsPart {}

impl SharedStringsPart {
    pub fn get_shared_string(&self, idx: usize) -> Option<&str> {
        self.strings
            .as_ref()
            .and_then(|ss| ss.get(idx))
            .map(|ss| ss.as_str())
    }
}

impl SharedString {
    pub fn as_str(&self) -> &str {
        match &self.t {
            Some(s) => s.as_str(),
            None => "",
        }
    }
}
