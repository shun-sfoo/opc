use std::fs;

use clap::{AppSettings, Clap};
use opc::document::sheet::SpreadsheetDocument;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "shun-sfoo <shun-sfoo@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    #[clap(short, validator = valid_filepath)]
    file_path: String,
}

fn valid_filepath(file_path: &str) -> Result<(), String> {
    if let Err(_) = fs::metadata(file_path) {
        return Err("file not exist".into());
    }

    Ok(())
}

fn main() {
    let opt = Opt::parse();

    let document = SpreadsheetDocument::open(opt.file_path).unwrap();
    let workbook = document.get_workbook();
    let _sheet_names = workbook.worksheet_names();
}
