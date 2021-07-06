use crate::packaging::element::OpenXmlDeserializeDefault;
use crate::packaging::namespace::Namespaces;

use serde::{Deserialize, Serialize};

use self::font::Fonts;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "styleSheet", rename_all = "camelCase")]
pub struct StylesPart {
    num_fmts: Option<NumberFormats>,
    fonts: Option<Fonts>,
    fills: Option<Fills>,
    cell_style_xfs: Option<CellStyleXfs>,
    cell_xfs: Option<CellXfs>,
    cell_styles: Option<CellStylesPart>,
    #[serde(flatten)]
    namespaces: Namespaces,
}

impl OpenXmlDeserializeDefault for StylesPart {}

impl StylesPart {
    pub fn get_cell_style(&self, id: usize) -> Option<&CellStyle> {
        self.cell_styles
            .as_ref()
            .and_then(|cs| cs.cell_style.get(id))
    }

    pub fn get_cell_style_component<'a>(&'a self, id: usize) -> Option<CellStyleComponent<'a>> {
        let cell_style = self.get_cell_style(id);
        cell_style.map(|cell_style| CellStyleComponent {
            styles: self,
            cell_style,
        })
    }

    pub fn get_cell_format_component<'a>(&'a self, id: usize) -> Option<CellFormatComponent<'a>> {
        let xf = self.get_cell_xf(id);
        xf.map(|xf| CellFormatComponent { styles: self, xf })
    }

    pub fn get_cell_xf(&self, id: usize) -> Option<&Xf> {
        self.cell_xfs.as_ref().and_then(|xf| {
            let xf1 = xf.xf.get(id);
            //let xf2 = xf.xf.iter().find()
            xf1
        })
    }

    pub fn get_number_format(&self, id: usize) -> Option<&NumberFormat> {
        self.num_fmts
            .as_ref()
            //.and_then(|inner| inner.num_fmt.get(id))
            .and_then(|inner| inner.num_fmt.as_ref())
            .and_then(|inner| inner.iter().find(|nf| nf.id == id))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "numFmts")]
pub struct NumberFormats {
    num_fmt: Option<Vec<NumberFormat>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "numFmt")]
pub struct NumberFormat {
    id: usize,
    pub code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellStyleXfs {
    count: usize,
    xf: Vec<Xf>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellXfs {
    count: usize,
    xf: Vec<Xf>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellStylesPart {
    count: usize,
    cell_style: Vec<CellStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellStyle {
    name: String,
    xf_id: usize,
    builtin_id: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xf {
    num_fmt_id: usize,
    font_id: usize,
    fill_id: usize,
    border_id: usize,
    apply_number_format: Option<bool>,
    apply_fill: Option<bool>,
    apply_alignment: Option<bool>,
    apply_protection: Option<bool>,
    alignment: Option<Alignment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alignment {
    vertical: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "fills")]
pub struct Fills {
    count: usize,
    #[serde(rename = "fill")]
    pub(crate) fills: Vec<Fill>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "fill")]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pattern_fill: Option<PatternFill>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "fgColor")]
#[serde(rename_all = "camelCase")]
pub struct PatternFill {
    pattern_type: Option<String>,
    bg_color: Option<BgColor>,
    fg_color: Option<FgColor>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "bgColor")]
#[serde(rename_all = "camelCase")]
pub struct BgColor {
    theme: Option<usize>,
    tint: Option<f64>,
    indexed: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "fgColor")]
#[serde(rename_all = "camelCase")]
pub struct FgColor {
    theme: Option<usize>,
    tint: Option<f64>,
    indexed: Option<usize>,
}

pub(crate) mod font {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "sz")]
    pub struct FontSize {
        val: f64,
    }
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "name")]
    pub struct FontName {
        val: String,
    }
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "charset")]
    pub struct FontCharset {
        val: String,
    }
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "scheme")]
    pub struct FontScheme {
        val: String,
    }
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "color")]
    pub struct FontColor {
        theme: Option<usize>,
        rbg: Option<String>,
    }
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "b")]
    pub struct FontBlack;
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "numFmt")]
    pub struct Font {
        black: Option<FontBlack>,
        #[serde(rename = "sz")]
        size: Option<FontSize>,
        /// the color theme id
        color: Option<FontColor>,
        name: String,
        charset: Option<String>,
        scheme: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    #[serde(rename = "fonts")]
    pub struct Fonts {
        #[serde(rename = "font")]
        pub(crate) fonts: Vec<Font>,
    }
}

#[derive(Debug)]
pub struct CellFormatComponent<'a> {
    styles: &'a StylesPart,
    xf: &'a Xf,
}

#[derive(Debug)]
pub struct CellStyleComponent<'a> {
    styles: &'a StylesPart,
    cell_style: &'a CellStyle,
}

impl<'a> CellFormatComponent<'a> {
    pub fn number_format(&self) -> Option<&NumberFormat> {
        self.styles.get_number_format(self.xf.num_fmt_id)
    }
}
