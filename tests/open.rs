#[cfg(test)]
mod tests {
    use opc::document::sheet::SpreadsheetDocument;
    use opc::packaging::package::OpenXmlPackage;

    #[test]
    #[ignore = "passed"]
    fn test_open_package() {
        let path = "resources/files/e1.xlsx";
        let package = OpenXmlPackage::open(path).unwrap();
        println!("{:?}", package);
    }

    #[test]
    fn test_document_open() {
        let path = "resources/files/e1.xlsx";
        let document = SpreadsheetDocument::open(path).unwrap();
        let workbook = document.get_workbook();

        let sheet_names = workbook.worksheet_names();
        println!("{:?}", sheet_names);

        for (sheet_idx, sheet) in workbook.worksheets().iter().enumerate() {
            println!("worksheet {}", sheet_idx);
            println!("worksheet dimension: {:?}", sheet.dimenstion());
            println!("---------DATA---------");
            for rows in sheet.rows() {
                // get cell values
                let cols: Vec<_> = rows
                    .into_iter()
                    .map(|cell| cell.value().unwrap_or_default())
                    .collect();
                println!("{}", itertools::join(&cols, ","));
            }
            println!("----------------------");
        }
    }
}
