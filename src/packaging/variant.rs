use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "vt:variant")]

pub enum Variant {
    #[serde(rename = "vt:vector")]
    VtVector {
        size: usize,
        #[serde(rename = "baseType")]
        base_type: String,
        #[serde(rename = "$value")]
        variants: Vec<Variant>,
    },
    #[serde(rename = "vt:variant")]
    VtVariant {
        #[serde(rename = "$value")]
        value: Box<Variant>,
    },
    #[serde(rename = "vt:null")]
    VtNull,
    #[serde(rename = "vt:i1")]
    VtI1(i8),
    #[serde(rename = "vt:i2")]
    VtI2(i8),
    #[serde(rename = "vt:i4")]
    VtI4(i8),
    #[serde(rename = "vt:i8")]
    VtI8(i8),
    #[serde(rename = "vt:lpstr")]
    VtLpstr(String),
    #[serde(rename = "vt:lpwstr")]
    VtLpwstr(String),
}

impl Default for Variant {
    fn default() -> Self {
        Variant::VtNull
    }
}
