use std::io::{Read, Seek};
use std::path::Path;

use crate::error::OoxmlError;

use linked_hash_map::LinkedHashMap;
use zip::ZipArchive;

use crate::packaging::{
    app_property::AppProperties, content_type::ContentTypes, custom_property::CustomProperties,
    part::OpenXmlPart, property::Properties, relationship::Relationships,
};

use crate::packaging::{
    app_property::APP_PROPERTIES_URI, content_type::CONTENT_TYPES_FILE,
    custom_property::CUSTOM_PROPERTIES_URI, property::CORE_PROPERTIES_URI,
    relationship::RELATIONSHIPS_FILE,
};

use crate::packaging::element::*;

#[derive(Debug, Clone, Default)]
pub struct OpenXmlPackage {
    content_types: ContentTypes,
    relationships: Relationships,
    app_properties: AppProperties,
    properties: Properties,
    coustom_properties: Option<CustomProperties>,
    parts: LinkedHashMap<String, OpenXmlPart>,
}

impl OpenXmlPackage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, OoxmlError> {
        let file = std::fs::File::open(path)?;
        Self::from_reader(file)
    }

    fn from_reader<R: Read + Seek>(reader: R) -> Result<Self, OoxmlError> {
        let mut zip = ZipArchive::new(reader)?;
        let mut package = OpenXmlPackage::default();
        let mut content_types_id = None;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            if file.is_dir() {
                continue;
            }

            let filename = file.name().to_string();
            if filename == CONTENT_TYPES_FILE {
                content_types_id = Some(i);
                let mut xml = String::new();
                file.read_to_string(&mut xml)?;
                package.content_types = ContentTypes::parse_from_xml_str(&xml);
                continue;
            } else if filename == RELATIONSHIPS_FILE {
                let mut xml = String::new();
                file.read_to_string(&mut xml)?;
                package.relationships = Relationships::parse_from_xml_str(&xml);
                continue;
            } else if filename == CORE_PROPERTIES_URI {
                let mut xml = String::new();
                file.read_to_string(&mut xml)?;
                package.properties = Properties::parse_from_xml_str(&xml);
                continue;
            } else if filename == CUSTOM_PROPERTIES_URI {
                let mut xml = String::new();
                file.read_to_string(&mut xml)?;
                package.coustom_properties = Some(CustomProperties::parse_from_xml_str(&xml));
            } else if filename == APP_PROPERTIES_URI {
                let mut xml = String::new();
                file.read_to_string(&mut xml)?;
                package.app_properties = OpenXmlDeserialized::from_xml_str(&xml).unwrap();
            } else {
                let uri = std::path::PathBuf::from(&filename);
                let part = OpenXmlPart::from_reader(uri, &mut file)?;
                package.parts.insert(filename, part);
            }
        }

        if content_types_id.is_none() {
            return Err(OoxmlError::PackageContentTypeError);
        }

        assert!(package.has_content_types());
        assert!(package.has_relationships());

        Ok(package)
    }

    pub fn get_part(&self, uri: &str) -> Option<&OpenXmlPart> {
        self.parts.get(uri)
    }

    pub fn has_content_types(&self) -> bool {
        !self.content_types.is_empty()
    }

    pub fn has_relationships(&self) -> bool {
        !self.relationships.is_empty()
    }
}
