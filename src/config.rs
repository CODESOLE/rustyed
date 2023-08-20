extern crate ini;
use ini::Ini;
use macroquad::prelude::Color;

#[derive(PartialEq, Eq, Debug)]
pub struct ConfigParseError;
pub struct Config {
    pub bg_col: Option<String>,
    pub font_col: Option<String>,
    pub font_size: Option<String>,
    pub font: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bg_col: None,
            font_col: None,
            font_size: None,
            font: None,
        }
    }
}

pub fn color_ascii_to_4u8(s: &str) -> Color {
    let rgba: Vec<&str> = s
        .trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim())
        .collect();

    let mut col: [u8; 4] = Default::default();
    for (i, &c) in rgba.iter().enumerate() {
        col[i] = c.parse::<u8>().expect("RGBA parsing error!");
    }

    Color::from_rgba(col[0], col[1], col[2], col[3])
}

pub fn parse_config(conf_path: &str) -> Result<Config, ini::Error> {
    let conf = Ini::load_from_file(conf_path)?;

    let editor = conf
        .section(Some("Editor"))
        .expect("'Editor' section could not be found!");

    let fontt = editor
        .get("font")
        .expect("'font' property could not be found!");
    let fontsize = editor
        .get("font_size")
        .expect("'font_size' property could not be found!");

    let style = conf
        .section(Some("Style"))
        .expect("'Style' section could not be found!");

    let bgcol = style
        .get("bg_col")
        .expect("'bg_col' property could not be found!");
    let fontcol = style
        .get("font_col")
        .expect("'font_col' property could not be found!");

    Ok(Config {
        bg_col: Some(bgcol.to_string()),
        font_col: Some(fontcol.to_string()),
        font_size: Some(fontsize.to_string()),
        font: Some(fontt.to_string()),
    })
}
