use crate::{
    buffer::Buffer,
    config::*,
    render::{from_str_to_cells, Cell},
};
use macroquad::prelude::*;
use std::path::PathBuf;

pub struct Context {
    pub mouse_pos: (f32, f32),
    pub curr_cursor_pos: (usize, usize),
    pub bg_color: Color,
    pub font: Font,
    pub font_color: Color,
    pub cursor_col: Color,
    pub font_size: u16,
    pub buffer: Buffer,
    pub cells: Vec<Cell>,
    pub active_buf: PathBuf,
    pub is_exit: bool,
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
            font_size: 10,
            buffer: Default::default(),
            cells: Default::default(),
            active_buf: Default::default(),
            is_exit: false,
        }
    }
}

pub async fn init(ctx: &mut Context, conf_path: &str, file: &PathBuf) {
    let conf = parse_config(conf_path).unwrap_or_default();
    if let Some(fnt) = &conf.font {
        ctx.font = load_ttf_font(fnt).await.unwrap_or_default();
    }
    if let Some(bgcol) = conf.bg_col {
        ctx.bg_color = color_ascii_to_4u8(&bgcol);
    }
    if let Some(foncol) = conf.font_col {
        ctx.font_color = color_ascii_to_4u8(&foncol);
    }
    if let Some(curcol) = conf.cursor_col {
        ctx.cursor_col = color_ascii_to_4u8(&curcol);
    }
    if let Some(fontsize) = conf.font_size {
        ctx.font_size = fontsize
            .parse::<u16>()
            .expect("Error happend while parsing font_size property!");
    }
    ctx.buffer = Buffer::new(file);
    ctx.buffer.read_to_buffer(file);
    ctx.active_buf = file.to_owned();

    from_str_to_cells(ctx);
}
