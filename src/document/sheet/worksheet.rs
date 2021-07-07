use crate::packaging::namespace::Namespaces;

use crate::document::sheet::cell::CellType;
use crate::packaging::element::{OpenXmlDeserializeDefault, OpenXmlDeserialized};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "worksheet")]
pub struct WorksheetPart {
    #[serde(flatten)]
    namespaces: Namespaces,
    pub sheet_pr: Option<SheetPr>,
    pub dimension: Option<Dimension>,
    pub sheet_views: Option<SheetViews>,
    pub sheet_format_pr: Option<SheetFormatPr>,
    pub cols: Option<SheetCols>,
    pub sheet_data: Option<SheetData>,
    pub page_margins: Option<PageMargins>,
    pub header_footer: Option<HeaderFooter>,
}

impl OpenXmlDeserializeDefault for WorksheetPart {}

impl WorksheetPart {
    pub fn dimenstion(&self) -> Option<(usize, usize)> {
        self.dimension.as_ref().and_then(|dim| dim.dimension())
    }

    pub fn real_dimenstion(&self) -> Option<(usize, usize)> {
        match (
            self.cols.as_ref().and_then(|cols| cols.cols.as_ref()),
            self.sheet_data.as_ref().and_then(|sd| sd.rows.as_ref()),
        ) {
            (Some(cols), Some(rows)) => Some((rows.len(), cols.len())),
            (Some(cols), None) => Some((0, cols.len())),
            (None, Some(rows)) => Some((rows.len(), 0)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheetPr")]
pub struct SheetPr {}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "dimension")]
pub struct Dimension {
    r#ref: String,
}

impl Dimension {
    pub fn dimension(&self) -> Option<(usize, usize)> {
        let range = self.r#ref.as_str();
        let range: Vec<&str> = range.split_terminator(':').collect();
        if range.len() < 2 {
            return None;
        }

        let start = range[0];
        let end = range[1];

        fn rangify(range: &str) -> (usize, usize) {
            let re: regex::Regex = regex::Regex::new(r"(?P<col>[A-Z]+)(?P<row>\d+)").unwrap();
            let cap = re.captures(range).unwrap();
            let col = cap.name("col").unwrap().as_str();
            let row = cap
                .name("row")
                .unwrap()
                .as_str()
                .parse()
                .unwrap_or_default();
            fn col_to_idx(col: &str) -> usize {
                if col.is_empty() {
                    return 0;
                }
                let mut idx = 0;
                for (i, c) in col.chars().rev().enumerate() {
                    let c = c.to_digit(36).unwrap();
                    idx += c * 26u32.pow(i as _);
                }
                return idx as usize;
            }
            (row, col_to_idx(col))
        }

        let start = rangify(start);
        let end = rangify(end);
        Some((end.0 - start.0 + 1, end.1 - start.1 + 1))
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheetViews")]
pub struct SheetViews {
    sheet_view: SheetView,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheetView")]
pub struct SheetView {
    tab_selected: Option<String>,
    workbook_view_id: Option<usize>,
    selection: Option<Selection>,
    show_formulas: Option<bool>,
    show_grid_lines: Option<bool>,
    show_row_col_headers: Option<bool>,
    show_zeros: Option<bool>,
    right_to_left: Option<bool>,
    show_outline_symbols: Option<bool>,
    default_grid_color: Option<String>,
    view: Option<String>,
    top_left_cell: Option<String>,
    color_id: Option<usize>,
    zoom_scale: Option<String>,
    zoom_scale_normal: Option<String>,
    zoom_scale_page_layout_view: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "calcPr")]
pub struct Selection {
    active_cell: String,
    sqref: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheetFormatPr")]
pub struct SheetFormatPr {
    default_col_width: Option<f32>,
    default_row_height: Option<f32>,
    outline_level_row: Option<f32>,
    outline_level_col: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "cols")]
pub struct SheetCols {
    pub cols: Option<Vec<SheetColHeader>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "col")]
pub struct SheetColHeader {
    min: Option<usize>,
    max: Option<usize>,
    width: Option<f64>,
    custom_width: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "sheetData")]
pub struct SheetData {
    #[serde(rename = "row")]
    pub rows: Option<Vec<SheetRow>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "row")]
pub struct SheetRow {
    pub r: usize,
    #[serde(rename = "customHeight")]
    pub custom_height: Option<bool>,
    pub ht: Option<f64>,
    pub spans: Option<String>,
    #[serde(rename = "c")]
    pub cols: Vec<SheetCol>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "c")]
pub struct SheetCol {
    pub r: String,
    pub t: Option<String>,
    pub s: Option<usize>,
    pub is: Option<SheetCellIs>,
    pub v: Option<String>,
}

impl SheetCol {
    pub fn as_raw_str(&self) -> &str {
        if let Some(is) = self.is.as_ref() {
            return is.t.as_ref().expect("inline str error");
        } else if let Some(v) = self.v.as_ref() {
            return v.as_str();
        } else {
            ""
        }
    }

    pub fn cell_type(&self) -> CellType {
        if self.t.is_none() && self.v.is_none() {
            return CellType::Empty;
        }

        match (self.t.as_ref(), self.s.as_ref()) {
            (None, None) => CellType::Raw,
            (Some(t), None) => match t {
                s if s == "s" => CellType::Shared(
                    self.v
                        .as_ref()
                        .expect("shared string has no id")
                        .parse()
                        .expect("sharedString id not valid"),
                ),
                n if n == "n" => CellType::Number,
                t if t == "inlineStr" => CellType::Raw,
                t => unimplemented!("cell type not supported: {}", t),
            },
            (None, Some(s)) => CellType::Styled(*s),
            (Some(t), Some(s)) => match t {
                t if t == "s" => CellType::Shared(
                    self.v
                        .as_ref()
                        .expect("shared string has no id")
                        .parse()
                        .expect("sharedString id not valid"),
                ),
                t if t == "n" => CellType::StyledNumber(*s),
                t if t == "inlineStr" => CellType::Raw,
                t => unimplemented!("cell type not supported: {}", t),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "is")]
pub struct SheetCellIs {
    t: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "pageMargins")]
pub struct PageMargins {
    left: Option<f32>,
    right: Option<f32>,
    top: Option<f32>,
    bottom: Option<f32>,
    header: Option<f32>,
    footer: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename = "headerFooter")]
pub struct HeaderFooter {}

#[test]
fn cell() {
    let xml = r#"
    <c r="B2" s="1" t="inlineStr">
        <is>
            <t>&#21776;&#33564;</t>
        </is>
    </c>"#;
    let c: SheetCol = quick_xml::de::from_str(xml).unwrap();
    println!("{:?}", c);
}
