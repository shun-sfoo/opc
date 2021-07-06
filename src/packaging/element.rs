use std::io::BufRead;

use crate::error::OoxmlError;

pub trait OpenXmlDeserialized: Sized {
    fn from_xml_reader<R: BufRead>(reader: R) -> Result<Self, OoxmlError>;

    fn from_xml_str(s: &str) -> Result<Self, OoxmlError> {
        Self::from_xml_reader(s.as_bytes())
    }
}

pub trait OpenXmlDeserializeDefault: serde::de::DeserializeOwned {}

impl<T: OpenXmlDeserializeDefault> OpenXmlDeserialized for T {
    fn from_xml_reader<R: BufRead>(reader: R) -> Result<Self, OoxmlError> {
        Ok(quick_xml::de::from_reader(reader)?)
    }
}
