use crate::{
    buffer::Buffer,
    config::*,
    render::{from_str_to_cells, Cell},
};
use copypasta::ClipboardContext;
use macroquad::prelude::*;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub enum Modes {
    GoToLine,
    FindCaseSensitive,
    FindCaseInSensitive,
    Edit,
    ModifiedPrompt,
    ShowHelp,
}

pub type SearchResults = Vec<(usize, (usize, usize))>;

pub struct Context {
    pub mouse_pos: (f32, f32),
    pub curr_cursor_pos: (usize, usize),
    pub bg_color: Color,
    pub font: Font,
    pub font_color: Color,
    pub cursor_col: Color,
    pub selection_col: Color,
    pub font_size: u16,
    pub buffer: Buffer,
    pub cells: Vec<Cell>,
    pub active_buf: PathBuf,
    pub is_exit: bool,
    pub is_cursorline: bool,
    pub vert_cell_count: (usize, usize),
    pub mode: Modes,
    pub prompt_input: String,
    pub search_res: SearchResults,
    pub is_search_changed: bool,
    pub last_searched_idx: usize,
    pub last_searched_term: String,
    pub is_font_monospaced: Option<f32>,
    pub is_file_changed: bool,
    pub tab_width: u8,
    pub eof_indicator: bool,
    pub selection_range: Option<((usize, (usize, usize)), (usize, (usize, usize)))>,
    pub clipboard: ClipboardContext,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            mouse_pos: mouse_position(),
            curr_cursor_pos: (0, 0),
            bg_color: color_u8!(0, 0, 0, 255),
            font: Default::default(),
            font_color: color_u8!(255, 255, 255, 255),
            cursor_col: color_u8!(200, 200, 200, 255),
            selection_col: color_u8!(55, 95, 25, 5),
            font_size: 10,
            buffer: Default::default(),
            is_cursorline: false,
            cells: Default::default(),
            active_buf: Default::default(),
            is_exit: false,
            vert_cell_count: (0, 10),
            mode: Modes::Edit,
            prompt_input: String::new(),
            search_res: Default::default(),
            is_search_changed: false,
            last_searched_idx: Default::default(),
            last_searched_term: String::new(),
            is_font_monospaced: None,
            is_file_changed: false,
            tab_width: 2,
            eof_indicator: false,
            selection_range: None,
            clipboard: ClipboardContext::new().expect("Failed when creating clipboard context!"),
        }
    }
}

fn is_font_monospaced(ctx: &Context) -> Option<f32> {
    let w = measure_text("m", Some(ctx.font), ctx.font_size, 1f32).width;
    if w == measure_text("i", Some(ctx.font), ctx.font_size, 1f32).width {
        println!("Monospace Font Detected!");
        Some(w)
    } else {
        println!("Non-Monospace Font Detected!");
        None
    }
}

pub async fn init(ctx: &mut Context, conf_path: &Path, file: &PathBuf) {
    let conf = parse_config(conf_path).unwrap_or_default();
    if let Some(fnt) = &conf.font {
        let fontt;
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(fnt)],
            weight: fontdb::Weight::NORMAL,
            ..fontdb::Query::default()
        };
        match db.query(&query) {
            Some(id) => {
                let (src, _) = db.face_source(id).unwrap();
                if let fontdb::Source::File(ref path) = &src {
                    fontt = load_ttf_font(&path.display().to_string()).await.unwrap();
                } else {
                    fontt = Font::default();
                }
            }
            None => {
                fontt = Font::default();
            }
        }
        ctx.font = fontt;
    }
    if let Some(bgcol) = conf.bg_col {
        ctx.bg_color = color_ascii_to_4u8(&bgcol);
    }
    if let Some(foncol) = conf.font_col {
        ctx.font_color = color_ascii_to_4u8(&foncol);
    }
    if let Some(selcol) = conf.select_col {
        ctx.selection_col = color_ascii_to_4u8(&selcol);
    }
    if let Some(curcol) = conf.cursor_col {
        ctx.cursor_col = color_ascii_to_4u8(&curcol);
    }
    if let Some(cur_line) = conf.cursor_line {
        let opt = cur_line
            .parse::<bool>()
            .expect("Error happened while parsing cursor_line property! It should be bool!");
        ctx.is_cursorline = opt;
    }
    if let Some(fontsize) = conf.font_size {
        ctx.font_size = fontsize
            .parse::<u16>()
            .expect("Error happend while parsing font_size property!");
    }
    if let Some(tabw) = conf.tab_width {
        ctx.tab_width = tabw
            .parse::<u8>()
            .expect("Error happend while parsing tab_width property!");
    }
    if let Some(eofindicator) = conf.eof_indicator {
        ctx.eof_indicator = eofindicator
            .parse::<bool>()
            .expect("Error happend while parsing eof_indicator property!");
    }
    ctx.buffer = Buffer::new(file);
    ctx.buffer.read_to_buffer(file);
    ctx.active_buf = file.to_owned();
    ctx.vert_cell_count = (0, screen_height() as usize / ctx.font_size as usize + 1);
    ctx.is_font_monospaced = is_font_monospaced(ctx);

    from_str_to_cells(ctx);
}
