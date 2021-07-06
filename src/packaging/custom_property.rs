use crate::packaging::{namespace::Namespaces, variant::Variant};

use serde::{Deserialize, Serialize};

pub const CUSTOM_PROPERTIES_URI: &str = "docProps/custom.xml";

pub const CUSTOM_PROPERTIES_TAG: &str = "Properties";
pub const CUSTOM_PROPERTIES_NAMESPACE_ATTRIBUTE: &str = "xmlns";
pub const CUSTOM_PROPERTIES_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";

pub const CUSTOM_PROPERTY_TAG: &str = "property";

pub const VT_NAMESPACE_ATTRIBUTE: &str = "xmlns:vt";
pub const VT_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Properties")]
pub struct CustomProperties {
    #[serde(flatten)]
    namespaces: Namespaces,
    #[serde(rename = "property")]
    properties: Vec<CustomProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all(deserialize = "camelCase"), rename = "property")]
pub struct CustomProperty {
    pub fmtid: String,
    pub pid: String,
    pub name: String,
    #[serde(rename = "$value")]
    value: Variant,
}

impl CustomProperties {
    pub fn parse_from_xml_str(reader: &str) -> Self {
        quick_xml::de::from_str(reader).unwrap()
    }
}
