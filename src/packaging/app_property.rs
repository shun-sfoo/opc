use super::namespace::Namespaces;
use super::variant::Variant;

use crate::packaging::element::OpenXmlDeserializeDefault;
use serde::{Deserialize, Serialize};

pub const APP_PROPERTIES_URI: &str = "docProps/app.xml";

pub const APP_PROPERTIES_TAG: &str = "Properties";
pub const APP_PROPERTIES_NAMESPACE_ATTRIBUTE: &str = "xmlns";
pub const APP_PROPERTIES_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/app-properties";

pub const APP_PROPERTY_TAG: &str = "property";

pub const VT_NAMESPACE_ATTRIBUTE: &str = "xmlns:vt";
pub const VT_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Properties", rename_all = "PascalCase")]
pub struct AppProperties {
    #[serde(flatten, skip_serializing)]
    pub namespaces: Namespaces,
    pub application: Option<Application>,
    pub heading_pairs: Option<HeadingPairs>,
    pub titles_of_parts: Option<TitlesOfParts>,
    pub lines: Option<String>,
    pub links_up_to_date: Option<String>,
    pub local_name: Option<String>,
    pub company: Option<String>,
    pub template: Option<String>,
    pub manager: Option<String>,
    pub pages: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingPairs {
    #[serde(rename = "$value")]
    variant: Variant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitlesOfParts {
    #[serde(rename(deserialize = "$value", serialize = "vt:vector"))]
    value: Variant,
}

impl AppProperties {
    pub fn parse_from_str(reader: &str) -> Self {
        quick_xml::de::from_str(reader).unwrap()
    }
}

impl OpenXmlDeserializeDefault for AppProperties {}
