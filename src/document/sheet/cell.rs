use std::fmt::Display;

use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CellType {
    Empty,
    Raw,
    Number,
    StyledNumber(usize),
    Shared(usize),
    Styled(usize),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CellValue {
    Null,
    Bool(bool),
    Int(i64),
    Byte(u8),
    Double(f64),
    String(String),
    DateTime(NaiveDateTime, String),
    Raw(String),
}

impl CellValue {
    pub fn to_string(&self) -> String {
        match self {
            CellValue::Null => "".to_string(),
            CellValue::String(v) => v.clone(),
            CellValue::Raw(v) => v.clone(),
            CellValue::Bool(_b) => panic!("unsupported cell type: bool"),
            CellValue::Double(f) => format!("{}", f),
            CellValue::DateTime(datetime, format) => format!("{}", datetime.format(&format)),
            _ => unimplemented!(),
        }
    }
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Null
    }
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
