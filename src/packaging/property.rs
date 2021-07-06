use super::content_type::ContentType;

use serde::Deserialize;
use serde::Serialize;
pub const CORE_PROPERTIES_URI: &str = "docProps/core.xml";
pub const CORE_PROPERTIES_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
pub const DC_NAMESPACE: &str = "http://purl.org/dc/elements/1.1/";
pub const DCTERMS_NAMESPACE: &str = "http://purl.org/dc/terms/";
pub const DCMITYPE_NAMESPACE: &str = "http://purl.org/dc/dcmitype/";
pub const XSI_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";

pub const CORE_PROPERTIES_TAG: &str = "cp:coreProperties";
pub const CORE_PROPERTIES_NAMESPACE_ATTRIBUTE: &str = "xmlns:dc";
pub const DC_NAMESPACE_ATTRIBUTE: &str = "xmlns:dc";
pub const DCTERMS_NAMESPACE_ATTRIBUTE: &str = "xmlns:dcterms";
pub const DCMITYPE_NAMESPACE_ATTRIBUTE: &str = "xmlns:dcmitype";
pub const XSI_NAMESPACE_ATTRIBUTE: &str = "xmlns:xsi";

pub const PROPERTY_CATEGORY_TAG: &str = "dc:creator";
pub const PROPERTY_CONTENT_STATUS_TAG: &str = "dc:contentStatus";
pub const PROPERTY_CONTENT_TYPE_TAG: &str = "dc:contentType";
pub const PROPERTY_CREATED_TAG: &str = "dcterms:created";
pub const PROPERTY_CREATOR_TAG: &str = "dc:creator";
pub const PROPERTY_DESCRIPTION_TAG: &str = "dc:description";
pub const PROPERTY_IDENTIFIER_TAG: &str = "dc:identifier";
pub const PROPERTY_KEYWORDS_TAG: &str = "dc:keywords";
pub const PROPERTY_LANGUAGE_TAG: &str = "dc:language";
pub const PROPERTY_MODIFIED_TAG: &str = "dcterms:modified";
pub const PROPERTY_LAST_MODIFIED_BY_TAG: &str = "cp:lastModifiedBy";
pub const PROPERTY_LAST_PRINTED_TAG: &str = "cp:lastPrinted";
pub const PROPERTY_REVISION_TAG: &str = "cp:revision";
pub const PROPERTY_SUBJECT_TAG: &str = "cp:subject";
pub const PROPERTY_TITLE_TAG: &str = "cp:title";
pub const PROPERTY_VERSION_TAG: &str = "cp:version";

pub type DateTime = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Properties {
    pub category: Option<String>,
    pub content_status: Option<String>,
    pub content_type: Option<ContentType>,
    pub created: Option<DateTime>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub keywords: Option<String>,
    pub language: Option<String>,
    pub modified: Option<String>,
    pub last_modified_by: Option<String>,
    pub last_printed: Option<DateTime>,
    pub revision: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub version: Option<String>,
}

impl Properties {
    pub fn parse_from_xml_str(reader: &str) -> Self {
        quick_xml::de::from_str(reader).unwrap()
    }
}
