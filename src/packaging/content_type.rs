use std::fmt::Display;

use linked_hash_map::LinkedHashMap;
use serde::de::{Deserialize, MapAccess, Visitor};

use crate::error::OoxmlError;

pub type ContentType = String;

pub const CONTENT_TYPES_FILE: &'static str = "[Content_Types].xml";
pub const TYPES_NAMESPACE_URI: &'static str =
    "http://schemas.openxmlformats.org/package/2006/content-types";
pub const TYPES_TAG_NAME: &'static str = "Types";
pub const DEFAULT_TAG_NAME: &'static str = "Default";
pub const OVERRIDE_TAG_NAME: &'static str = "Override";
pub const PART_NAME_ATTRIBUTE_NAME: &'static str = "PartName";
pub const EXTENSION_ATTRIBUTE_NAME: &'static str = "Extension";
pub const CONTENT_TYPE_ATTRIBUTE_NAME: &'static str = "ContentType";
pub const XMLNS_ATTRIBUTE_NAME: &'static str = "xmlns";

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ContentTypes {
    defaults: LinkedHashMap<String, ContentType>,
    overrides: LinkedHashMap<String, ContentType>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Default {
    extension: String,
    content_type: ContentType,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Override {
    part_name: String,
    content_type: ContentType,
}

impl Display for ContentTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut container = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut container);
        self.write(&mut cursor).expect("write xml to memory error");
        let s = String::from_utf8_lossy(&container);
        write!(f, "{}", s)?;
        Ok(())
    }
}
impl ContentTypes {
    pub fn parse_from_xml_str(reader: &str) -> Self {
        quick_xml::de::from_str(reader).unwrap()
    }

    pub fn add_default_element(&mut self, extension: String, content_type: ContentType) {
        self.defaults.insert(extension, content_type);
    }

    pub fn add_override_element(&mut self, part_name: String, content_type: ContentType) {
        self.overrides.insert(part_name, content_type);
    }

    pub fn write<W: std::io::Write>(&self, writer: W) -> Result<(), OoxmlError> {
        let mut xml = quick_xml::Writer::new(writer);
        use quick_xml::events::attributes::Attribute;
        use quick_xml::events::*;

        // 1. write decl
        xml.write_event(Event::Decl(BytesDecl::new(
            b"1.0",
            Some(b"UTF-8"),
            Some(b"yes"),
        )))?;

        // 2. start types element
        let mut elem = BytesStart::borrowed_name(TYPES_TAG_NAME.as_bytes());
        let ns = Attribute {
            key: XMLNS_ATTRIBUTE_NAME.as_bytes(),
            value: TYPES_NAMESPACE_URI.as_bytes().into(),
        };
        elem.extend_attributes(vec![ns]);
        xml.write_event(Event::Start(elem))?;

        // 3. write default entries
        for (key, value) in &self.defaults {
            xml.write_event(Event::Empty(
                BytesStart::borrowed_name(DEFAULT_TAG_NAME.as_bytes()).with_attributes(vec![
                    Attribute {
                        key: EXTENSION_ATTRIBUTE_NAME.as_bytes(),
                        value: key.as_bytes().into(),
                    },
                    Attribute {
                        key: CONTENT_TYPE_ATTRIBUTE_NAME.as_bytes(),
                        value: value.as_bytes().into(),
                    },
                ]),
            ))?;
        }

        // 4. write override entries
        for (key, value) in &self.overrides {
            xml.write_event(Event::Empty(
                BytesStart::borrowed_name(OVERRIDE_TAG_NAME.as_bytes()).with_attributes(vec![
                    Attribute {
                        key: PART_NAME_ATTRIBUTE_NAME.as_bytes(),
                        value: key.as_bytes().into(),
                    },
                    Attribute {
                        key: CONTENT_TYPE_ATTRIBUTE_NAME.as_bytes(),
                        value: value.as_bytes().into(),
                    },
                ]),
            ))?;
        }

        // 5 ends types element.
        let end = BytesEnd::borrowed(TYPES_TAG_NAME.as_bytes());
        xml.write_event(Event::End(end))?;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.defaults.is_empty() && self.overrides.is_empty()
    }
}

struct ContentTypesVisitor;

impl<'de> Visitor<'de> for ContentTypesVisitor {
    type Value = ContentTypes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut types = ContentTypes::default();
        while let Some(key) = access.next_key()? {
            let key: String = key;
            match &key {
                s if s == XMLNS_ATTRIBUTE_NAME => {
                    let _xmlns: String = access.next_value()?;
                }
                s if s == TYPES_TAG_NAME => {
                    unreachable!();
                }
                s if s == DEFAULT_TAG_NAME => {
                    let v: Default = access.next_value()?;
                    types.add_default_element(v.extension, v.content_type);
                }
                s if s == OVERRIDE_TAG_NAME => {
                    let v: Override = access.next_value()?;
                    types.add_override_element(v.part_name, v.content_type);
                }
                _ => {
                    unreachable!("content type unsupport!");
                }
            }
        }
        Ok(types)
    }
}

impl<'de> Deserialize<'de> for ContentTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ContentTypesVisitor)
    }
}

#[test]
#[ignore = "pass"]
fn test_de() {
    let raw = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="png" ContentType="image/png"/><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/><Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/><Override PartName="/docProps/custom.xml" ContentType="application/vnd.openxmlformats-officedocument.custom-properties+xml"/><Override PartName="/xl/charts/chart1.xml" ContentType="application/vnd.openxmlformats-officedocument.drawingml.chart+xml"/><Override PartName="/xl/charts/colors1.xml" ContentType="application/vnd.ms-office.chartcolorstyle+xml"/><Override PartName="/xl/charts/style1.xml" ContentType="application/vnd.ms-office.chartstyle+xml"/><Override PartName="/xl/drawings/drawing1.xml" ContentType="application/vnd.openxmlformats-officedocument.drawing+xml"/><Override PartName="/xl/drawings/drawing2.xml" ContentType="application/vnd.openxmlformats-officedocument.drawing+xml"/><Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/><Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/><Override PartName="/xl/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#;

    println!("{}", raw);

    let content_types: ContentTypes = quick_xml::de::from_str(raw).unwrap();

    println!("{:?}", content_types);
    let display = format!("{}", content_types);
    assert_eq!(raw, display);
}
