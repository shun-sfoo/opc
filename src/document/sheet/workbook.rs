use crate::packaging::{element::OpenXmlDeserializeDefault, namespace::Namespaces};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbookPart {
    pub file_version: Option<FileVersion>,
    pub book_views: BookViews,
    pub workbook_pr: WorkbookPr,
    pub sheets: Sheets,
    pub calc_pr: Option<CalcPr>,
    #[serde(flatten)]
    namespaces: Namespaces,
}

impl OpenXmlDeserializeDefault for WorkbookPart {}

impl WorkbookPart {
    pub fn sheet_names(&self) -> Vec<&str> {
        self.sheets
            .sheets
            .iter()
            .map(|sheet| sheet.name.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "fileVersion")]
pub struct FileVersion {
    pub app_name: Option<String>,
    pub last_edited: Option<usize>,
    pub lowest_edited: Option<usize>,
    pub run_build: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "bookViews")]
pub struct BookViews {
    pub workbook_view: WorkbookView,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "workbookView")]
pub struct WorkbookView {
    pub window_width: Option<usize>,
    pub window_height: Option<usize>,
    pub active_tab: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "workbookPr")]
pub struct WorkbookPr {
    date1904: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheets")]
pub struct Sheets {
    #[serde(rename = "sheet")]
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheet")]
pub struct Sheet {
    pub name: String,
    pub sheet_id: usize,
    #[serde(rename = "r:id")]
    pub r_id: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "calcPr")]
pub struct CalcPr {
    calc_id: Option<String>,
}
