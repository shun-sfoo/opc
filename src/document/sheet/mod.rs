use crate::document::sheet::cell::CellValue;
use crate::document::sheet::style::CellFormatComponent;
use crate::document::sheet::worksheet::SheetCol;
use crate::error::Result;
use crate::packaging::element::OpenXmlDeserialized;
use crate::packaging::{package::OpenXmlPackage, relationship::Relationships};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

mod cell;
mod document_type;
mod shared_string;
mod style;
mod workbook;
mod worksheet;

use self::{
    document_type::SpreadsheetDocumentType, shared_string::SharedStringsPart, style::StylesPart,
    workbook::WorkbookPart, worksheet::WorksheetPart,
};

#[derive(Default, Debug)]
pub struct SpreadsheetDocument {
    package: Rc<RefCell<OpenXmlPackage>>,
    parts: Rc<RefCell<SpreadsheetParts>>,
    document_type: SpreadsheetDocumentType,
    workbook: Workbook,
}

impl SpreadsheetDocument {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let package = OpenXmlPackage::open(path)?;
        let package = Rc::new(RefCell::new(package));
        let parts = SpreadsheetParts::from_package(package.clone());
        let parts = Rc::new(RefCell::new(parts));
        let workbook = Workbook::new(parts.clone());
        let document_type = SpreadsheetDocumentType::Workbook;
        Ok(Self {
            package,
            parts,
            workbook,
            document_type,
        })
    }

    pub fn get_workbook(&self) -> &Workbook {
        &self.workbook
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpreadsheetParts {
    initialized: bool,
    pub package: Rc<RefCell<OpenXmlPackage>>,
    pub relationships: Relationships,
    pub workbook: WorkbookPart,
    pub styles: StylesPart,
    pub shared_strings: SharedStringsPart,
    pub worksheets: linked_hash_map::LinkedHashMap<String, WorksheetPart>,
}

impl SpreadsheetParts {
    pub fn parse_worksheets(&mut self) {
        for sheet in &self.workbook.sheets.sheets {
            let relationship = self
                .relationships
                .get_relationship_by_id(&sheet.r_id)
                .expect("the worksheet relationship does not exist");
            let worksheet_uri = relationship.target();
            let package = self.package.borrow();
            let part = package
                .get_part(&format!("xl/{}", worksheet_uri))
                .expect("get worksheet part by uri");
            let sheet = WorksheetPart::from_xml_reader(part.as_part_bytes())
                .expect("parse worksheet error");

            self.worksheets.insert(worksheet_uri.into(), sheet);
        }
    }

    pub fn get_worksheet_part<T: AsRef<str>>(&self, uri: T) -> Option<&WorksheetPart> {
        self.worksheets.get(uri.as_ref())
    }

    pub fn sheet_names(&self) -> Vec<&str> {
        self.workbook.sheet_names()
    }

    pub fn get_shared_string(&self, idx: usize) -> Option<&str> {
        self.shared_strings.get_shared_string(idx)
    }

    pub fn get_cell_format<'a>(&'a self, id: usize) -> Option<CellFormatComponent<'a>> {
        self.styles.get_cell_format_component(id)
    }
}

impl SpreadsheetParts {
    pub fn from_package(package: Rc<RefCell<OpenXmlPackage>>) -> Self {
        let relationships = {
            let package = package.borrow();
            let part = package.get_part("xl/_rels/workbook.xml.rels").unwrap();
            Relationships::parse_from_xml_reader(part.as_part_bytes())
        };

        let workbook = {
            let package = package.borrow();
            let part = package.get_part("xl/workbook.xml").unwrap();
            WorkbookPart::from_xml_reader(part.as_part_bytes()).expect("workbook main part error")
        };

        let shared_strings = {
            let package = package.borrow();
            let part = package.get_part("xl/sharedStrings.xml").unwrap();
            SharedStringsPart::from_xml_reader(part.as_part_bytes())
                .expect("workbook shared strings error")
        };

        let styles = {
            let package = package.borrow();
            let part = package.get_part("xl/styles.xml").unwrap();
            StylesPart::from_xml_reader(part.as_part_bytes()).expect("workbook styles error")
        };

        let mut this = Self {
            package,
            relationships,
            workbook,
            shared_strings,
            styles,
            initialized: true,
            ..Default::default()
        };
        this.parse_worksheets();
        this
    }
}

#[derive(Default, Debug, Clone)]
pub struct Workbook {
    parts: Rc<RefCell<SpreadsheetParts>>,
    worksheets: Vec<Worksheet>,
}

impl Workbook {
    pub fn new(parts: impl Into<Rc<RefCell<SpreadsheetParts>>>) -> Self {
        let parts = parts.into();
        let borrowed_parts = parts.borrow();
        let mut worksheets = Vec::new();

        for sheet in &borrowed_parts.workbook.sheets.sheets {
            let relationship = borrowed_parts
                .relationships
                .get_relationship_by_id(&sheet.r_id)
                .expect("the worksheet relationship does not exist");
            let worksheet_uri = relationship.target();

            let part = borrowed_parts.get_worksheet_part(&worksheet_uri).unwrap();

            let worksheet = Worksheet {
                parts: parts.clone(),
                name: sheet.name.clone(),
                sheet_id: sheet.sheet_id,
                part: part.clone(),
            };

            worksheets.push(worksheet);
        }

        Self {
            parts: parts.clone(),
            worksheets,
        }
    }

    pub fn worksheet_names(&self) -> Vec<String> {
        self.parts
            .borrow()
            .sheet_names()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn worksheets(&self) -> &[Worksheet] {
        self.worksheets.as_slice()
    }
}

#[derive(Debug, Clone)]
pub struct Worksheet {
    parts: Rc<RefCell<SpreadsheetParts>>,
    name: String,
    sheet_id: usize,
    part: WorksheetPart,
}

impl Worksheet {
    pub fn dimenstion(&self) -> Option<(usize, usize)> {
        self.part
            .dimenstion()
            .or_else(|| self.part.real_dimenstion())
    }

    pub fn rows<'a>(&'a self) -> RowsIter<'a> {
        RowsIter {
            sheet: self,
            row: 0,
            col: 0,
        }
    }

    pub fn get_row_size(&self) -> usize {
        self.dimenstion().unwrap_or_default().0
    }

    pub fn get_col_size(&self) -> usize {
        self.dimenstion().unwrap_or_default().0
    }

    pub fn get_shared_string(&self, idx: usize) -> Option<String> {
        let parts = self.parts.as_ref().borrow();
        parts.get_shared_string(idx).map(|s| s.into())
    }

    pub fn to_cell_value(&self, raw: &str, style_id: usize) -> Option<CellValue> {
        let parts = self.parts.as_ref().borrow();
        let cs = parts.get_cell_format(style_id);
        let cs = cs.unwrap();
        let nf = cs.number_format();
        if nf.is_none() {
            dbg!(raw, style_id);
            return Some(CellValue::String(raw.to_string()));
        }
        let nf = nf.unwrap();
        let code = nf.code.as_str();

        fn parse_datetime(raw: &str) -> Option<chrono::NaiveDateTime> {
            if let Ok(days) = raw.parse::<i64>() {
                let days = days - 25569;
                let secs = days * 86400;
                chrono::NaiveDateTime::from_timestamp_opt(secs, 0)
            } else if let Ok(datetime) = raw.parse::<f64>() {
                let unix_days = datetime - 25569.;
                let unix_secs = unix_days * 86400.;
                let secs = unix_secs.trunc() as i64;
                let nsecs = (unix_secs.fract().abs() * 1e9) as u32;
                chrono::NaiveDateTime::from_timestamp_opt(secs, nsecs)
            } else {
                None
            }
        }
        let datetime_re = regex::Regex::new("y{1,4}|m{1,5}|d|h|ss|a{2,5}").unwrap();

        let datetime_replaces = vec![
            (regex::Regex::new(":mm").unwrap(), ":%M"),
            (regex::Regex::new("mm:").unwrap(), "%M:"),
            (regex::Regex::new("mm").unwrap(), "%m"),
            (regex::Regex::new("yyyy+").unwrap(), "%Y"),
            (regex::Regex::new("yy+").unwrap(), "%y"),
            (regex::Regex::new("mmmm+").unwrap(), "%B"),
            (regex::Regex::new("mmm").unwrap(), "%b"),
            (regex::Regex::new("([^%]|^)m").unwrap(), "$1%m"),
            (regex::Regex::new("d+").unwrap(), "%d"),
            (regex::Regex::new("a{4,}").unwrap(), "%A"),
            (regex::Regex::new("a{3}").unwrap(), "%a"),
            (regex::Regex::new("a{2}").unwrap(), "%w"),
            (regex::Regex::new("h").unwrap(), "%H"),
            (regex::Regex::new("ss").unwrap(), "%S"),
            (regex::Regex::new("\\\\").unwrap(), ""),
        ];
        let s = match code {
            s if s == "General" => CellValue::String(raw.to_string()),
            format if datetime_re.is_match(format) | format.ends_with(";@") => {
                // dbg!(&format);
                let format = format.trim_end_matches(";@");
                let datetime = parse_datetime(raw).unwrap();

                let format = datetime_replaces
                    .iter()
                    .fold(snailquote::unescape(format).unwrap(), |f, (re, s)| {
                        re.replace_all(&f, *s).to_string()
                    });
                // dbg!(&format);
                CellValue::DateTime(datetime, format)
                // format!("{}", datetime.format(&format))
            }
            s => {
                // FIXME(@zitsen): support custom format like dollars, etc.
                eprintln!("unimplemented format support: {}", s);
                CellValue::String(raw.to_string())
            }
        };
        Some(s)
    }
}

#[derive(Debug)]
pub struct RowsIter<'a> {
    sheet: &'a Worksheet,
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct RowIter<'a> {
    sheet: &'a Worksheet,
    row: usize,
    col: usize,
}

impl<'a> Iterator for RowsIter<'a> {
    type Item = RowIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.sheet.get_row_size() {
            return None;
        };
        let row = self.row_iter();
        self.row += 1;
        Some(row)
    }
}

impl<'a> RowsIter<'a> {
    fn row_iter(&self) -> RowIter<'a> {
        RowIter {
            sheet: self.sheet,
            row: self.row,
            col: self.col,
        }
    }
}

pub struct Cell<'a> {
    sheet: &'a Worksheet,
    row: usize,
    col: usize,
}

impl<'a> Cell<'a> {
    fn inner(&self) -> Option<&SheetCol> {
        let data = self.sheet.part.sheet_data.as_ref().unwrap();
        data.rows
            .as_ref()
            .and_then(|rows| rows.get(self.row))
            .and_then(|row| row.cols.get(self.col))
    }

    pub fn value(&self) -> Option<CellValue> {
        let inner = self.inner();
        if inner.is_none() {
            return None;
        }
        let inner = inner.unwrap();
        let raw = inner.as_raw_str();
        let ctype = inner.cell_type();
        let value = match ctype {
            cell::CellType::Empty => CellValue::Null,
            cell::CellType::Raw => CellValue::String(raw.to_string()),
            cell::CellType::Number => CellValue::String(raw.to_string()),
            cell::CellType::Shared(shared_string_id) => CellValue::String(
                self.sheet
                    .get_shared_string(shared_string_id)
                    .expect(&format!("shared string not found {}", shared_string_id)),
            ),
            cell::CellType::Styled(style_id) => self
                .sheet
                .to_cell_value(&inner.as_raw_str(), style_id)
                .expect("format with cell style"),
            cell::CellType::StyledNumber(style_id) => self
                .sheet
                .to_cell_value(&inner.as_raw_str(), style_id)
                .expect("format with cell style"),
        };
        Some(value)
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Cell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col >= self.sheet.get_col_size() {
            return None;
        };
        let cell = Cell {
            sheet: self.sheet,
            row: self.row,
            col: self.col,
        };
        self.col += 1;
        Some(cell)
    }
}
