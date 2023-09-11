use macroquad::prelude::Color;
use std::{collections::HashMap, error::Error, io::Read, path::Path};

#[derive(PartialEq, Eq, Debug)]
pub struct ConfigParseError;

#[derive(Debug)]
pub struct Config {
    pub bg_col: Option<String>,
    pub font_col: Option<String>,
    pub font_size: Option<String>,
    pub font: Option<String>,
    pub cursor_col: Option<String>,
    pub cursor_line: Option<String>,
    pub tab_width: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bg_col: None,
            font_col: None,
            font_size: None,
            font: None,
            cursor_col: None,
            cursor_line: None,
            tab_width: None,
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

pub fn parse_config(conf_path: &Path) -> Result<Config, Box<dyn Error>> {
    let mut file_content = String::new();
    let mut file = std::fs::File::open(conf_path)?;
    let _ = file.read_to_string(&mut file_content);
    let mut pairs = HashMap::new();
    let mut config = Config::default();

    for l in file_content.lines() {
        let s: &str;
        if let Some(idx) = l.find(';') {
            s = &l[..idx];
        } else {
            s = l;
        }
        if s.trim().is_empty() {
            continue;
        }
        if let Some((k, v)) = s
            .trim()
            .split_once('=')
            .and_then(|(k, v)| Some((k.trim().to_owned(), v.trim().to_owned())))
        {
            pairs.insert(k, v);
        } else {
            return Err("Parse Error!".into());
        }
    }
    config.font = pairs.get("font").cloned().take();
    config.font_col = pairs.get("font_col").cloned().take();
    config.font_size = pairs.get("font_size").cloned().take();
    config.bg_col = pairs.get("bg_col").cloned().take();
    config.cursor_col = pairs.get("cursor_col").cloned().take();
    config.cursor_line = pairs.get("cursor_line").cloned().take();
    config.tab_width = pairs.get("tab_width").cloned().take();

    Ok(config)
}
