#[derive(Debug, Clone, Copy)]
pub enum SpreadsheetDocumentType {
    // excel workbook (*.xlsx).
    Workbook,
    // Excel Template (*.xlsx).
    Template,
    // Excel Macro-Enabled Workbook (*.xlsm).
    MacroEnabledWorkbook,
    // Excel Macro-Enabled Template (*.xltm).
    MacroEnabledTemplate,
    // Excel Add-In (*.xlam).
    AddIn,
}

impl Default for SpreadsheetDocumentType {
    fn default() -> Self {
        SpreadsheetDocumentType::Workbook
    }
}
