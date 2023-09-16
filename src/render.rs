use macroquad::{prelude::*, window};

use crate::core::{Context, Modes};

pub const HELP_PAGE: &str = "

        HELP PAGE

ESC ==> Exit from help page, search mode, gotoline mode.

CTRL - Q ==> Quit from application.

CTRL - S ==> Save document.

CTRL - O ==> Open Document.

PageUp/Down ==> PageUp/PageDown.

CTRL - PageUp/Down ==> Goto top/bottom of document.

CTRL - G ==> Go To Line mode.

CTRL - H ==> Open help page.

CTRL - F ==> Search document in case sensitive mode.

CTRL - Shift - F ==> Search document in case insensitive mode.

Home ==> Go to begining of line.

End ==> Go to end of line.

Left/Right Arrow ==> Move cursor by one char left/right.

CTRL - Left/Right Arrow ==> Move cursor by word.

Up/Down Arrow ==> Move cursor Up/Down.

Delete ==> Delete char under cursor.

Backspace ==> Remove previous char.

CTRL - C/X/V ==> Copy/Cut/Paste operation.

CTRL - W ==> Delete previous word surround by space.

CTRL - Z/Y ==> Undo/Redo.";

#[derive(Default, Debug)]
pub struct Cell {
    pub c: char,
    pub coord: (f32, f32),
    pub bound: (f32, f32),
    pub pos: (usize, usize),
}

pub fn from_str_to_cells(ctx: &mut Context) {
    let mut cells: Vec<Cell> = Default::default();
    let mut x_coor;
    let mut y_coor = 0f32;
    let end_rng = std::cmp::min(
        ctx.vert_cell_count.0 + ctx.vert_cell_count.1,
        ctx.buffer.buf.lines().count(),
    );

    ctx.buffer.vec_str.clear();
    for s in ctx.buffer.buf.lines() {
        let mut ss = s.to_string();
        ss.push('\n');
        ctx.buffer.vec_str.push(ss);
    }

    let mut prev_lf_idx = 0usize;
    for (line_idx, (lf_idx, _)) in ctx.buffer.buf.match_indices('\n').enumerate() {
        if !(ctx.vert_cell_count.0..end_rng).contains(&line_idx) {
            continue;
        }
        let line;
        if line_idx == 0 {
            line = &ctx.buffer.buf[0..=lf_idx];
        } else {
            line = &ctx.buffer.buf[prev_lf_idx + 1..=lf_idx];
        }
        if let Some(w) = ctx.is_font_monospaced {
            for (j, ch) in line.chars().enumerate() {
                x_coor = j as f32 * w;
                cells.push(Cell {
                    c: ch,
                    coord: (x_coor, y_coor),
                    bound: (w, ctx.font_size as f32),
                    pos: (j, line_idx),
                });
            }
        } else {
            for (j, ch) in line.chars().enumerate() {
                let letter_size =
                    measure_text(&ch.to_string(), Some(ctx.font), ctx.font_size, 1.0f32);
                x_coor = measure_text(&line[..j], Some(ctx.font), ctx.font_size, 1f32).width;
                cells.push(Cell {
                    c: ch,
                    coord: (x_coor, y_coor),
                    bound: (letter_size.width, ctx.font_size as f32),
                    pos: (j, line_idx),
                });
            }
        }
        y_coor = (line_idx + 1) as f32 * ctx.font_size as f32;
        prev_lf_idx = lf_idx;
    }
    ctx.cells = cells;
}

pub fn from_cells_to_string(cells: &Vec<Cell>) -> Vec<String> {
    let mut s = String::new();
    let mut vec_of_string = Vec::<String>::new();
    cells.iter().for_each(|c| s.push(c.c));
    for l in s.lines() {
        vec_of_string.push(l.to_owned());
        vec_of_string.iter_mut().last().unwrap().push('\n');
    }
    vec_of_string
}

fn draw_cursor_location(ctx: &Context) {
    let loc_str = format!(
        "{} {}:{}:{}",
        if ctx.is_file_changed { "[+] " } else { "" },
        ctx.active_buf.file_name().unwrap().to_str().unwrap(),
        ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 + 1,
        ctx.curr_cursor_pos.0 + 1
    );
    let offset = measure_text(loc_str.as_str(), Some(ctx.font), ctx.font_size, 1f32).width;
    let win_width = window::screen_width();
    let x = win_width - offset;
    draw_rectangle(
        x,
        0f32,
        offset,
        ctx.font_size as f32,
        color_u8!(255, 0, 0, 255),
    );
    draw_text_ex(
        loc_str.as_str(),
        x,
        12f32,
        TextParams {
            font_size: ctx.font_size,
            color: color_u8!(0, 0, 0, 255),
            font: ctx.font,
            ..Default::default()
        },
    );
}

fn draw_cursor_line(ctx: &Context, cursor: &Cell) {
    let width = macroquad::window::screen_width();
    draw_rectangle(
        0f32,
        cursor.coord.1,
        width,
        ctx.font_size as f32,
        color_u8!(255, 255, 255, 10),
    );
}

pub fn draw_go_to_prompt(ctx: &Context, line: &str) {
    let (win_w, win_h) = (screen_width(), screen_height());
    draw_rectangle(
        0f32,
        win_h - ctx.font_size as f32,
        win_w,
        ctx.font_size as f32,
        color_u8!(255, 0, 0, 255),
    );
    draw_text_ex(
        &format!(" Line Number: {}", line),
        0f32,
        win_h - ctx.font_size as f32,
        TextParams {
            font_size: ctx.font_size,
            color: color_u8!(0, 0, 0, 255),
            font: ctx.font,
            ..Default::default()
        },
    );
}

pub fn draw_find_prompt(ctx: &Context, line: &str, is_case_sensitive: bool) {
    let (win_w, win_h) = (screen_width(), screen_height());
    draw_rectangle(
        0f32,
        win_h - ctx.font_size as f32,
        win_w,
        ctx.font_size as f32,
        color_u8!(255, 0, 0, 255),
    );
    if is_case_sensitive {
        draw_text_ex(
            &format!(
                " Find(CaseSensitive): {}  [{}/{}]",
                line,
                if !ctx.search_res.is_empty() {
                    ctx.last_searched_idx + 1
                } else {
                    ctx.last_searched_idx
                },
                ctx.search_res.len()
            ),
            0f32,
            win_h - ctx.font_size as f32 + 12f32,
            TextParams {
                font_size: ctx.font_size,
                color: color_u8!(0, 0, 0, 255),
                font: ctx.font,
                ..Default::default()
            },
        );
    } else {
        draw_text_ex(
            &format!(
                " Find(CaseInSensitive): {}  [{}/{}]",
                line,
                if !ctx.search_res.is_empty() {
                    ctx.last_searched_idx + 1
                } else {
                    ctx.last_searched_idx
                },
                ctx.search_res.len()
            ),
            0f32,
            win_h - ctx.font_size as f32 + 12f32,
            TextParams {
                font_size: ctx.font_size,
                color: color_u8!(0, 0, 0, 255),
                font: ctx.font,
                ..Default::default()
            },
        );
    }
}

pub fn draw_unsaved_prompt(ctx: &Context) {
    let (win_w, win_h) = (screen_width(), screen_height());
    draw_rectangle(
        0f32,
        win_h - ctx.font_size as f32,
        win_w,
        ctx.font_size as f32,
        color_u8!(255, 0, 0, 255),
    );
    draw_text_ex(
        "File is modified! Are you sure to quit?[Press 'y' to exit without saving, press 'n' to not exit!]",
        0f32,
        win_h - ctx.font_size as f32 + 12f32,
        TextParams {
            font_size: ctx.font_size,
            color: color_u8!(0, 0, 0, 255),
            font: ctx.font,
            ..Default::default()
        },
    );
}

fn draw_eof_indicator(ctx: &Context) {
    if (ctx.vert_cell_count.0 + ctx.vert_cell_count.1) > ctx.buffer.buf.lines().count() {
        let y = ctx.cells.iter().last().unwrap().coord.1 + ctx.font_size as f32;
        let w = measure_text("EOF", Some(ctx.font), ctx.font_size, 1f32).width;
        draw_rectangle(
            0f32,
            y,
            w,
            ctx.font_size as f32,
            color_u8!(255, 255, 255, 100),
        );
        draw_text_ex(
            "EOF",
            0f32,
            y + 12f32,
            TextParams {
                font_size: ctx.font_size,
                color: color_u8!(0, 0, 0, 255),
                font: ctx.font,
                ..Default::default()
            },
        );
    }
}

pub async fn render(ctx: &Context) {
    clear_background(ctx.bg_color);
    if ctx.eof_indicator {
        draw_eof_indicator(ctx);
    }
    let cursor_to_render = ctx
        .cells
        .iter()
        .filter(|c| c.pos == ctx.curr_cursor_pos)
        .next()
        .unwrap();
    if ctx.is_cursorline {
        draw_cursor_line(ctx, cursor_to_render);
    }
    for cell in ctx.cells.iter() {
        if cell.c == '\n' {
            continue;
        }
        draw_text_ex(
            &cell.c.to_string(),
            cell.coord.0,
            cell.coord.1 + 12f32,
            TextParams {
                font_size: ctx.font_size,
                color: ctx.font_color,
                font: ctx.font,
                ..Default::default()
            },
        );
    }
    draw_rectangle(
        cursor_to_render.coord.0,
        cursor_to_render.coord.1,
        cursor_to_render.bound.0,
        cursor_to_render.bound.1,
        ctx.cursor_col,
    );
    draw_cursor_location(ctx);
    if ctx.mode == Modes::GoToLine {
        draw_go_to_prompt(ctx, &ctx.prompt_input);
    } else if ctx.mode == Modes::FindCaseSensitive {
        draw_find_prompt(ctx, &ctx.prompt_input, true);
    } else if ctx.mode == Modes::FindCaseInSensitive {
        draw_find_prompt(ctx, &ctx.prompt_input, false);
    } else if ctx.mode == Modes::ModifiedPrompt {
        draw_unsaved_prompt(ctx);
    } else if ctx.mode == Modes::ShowHelp {
        render_help_page(ctx);
    }
    next_frame().await
}

fn render_help_page(ctx: &Context) {
    let (win_w, win_h) = (screen_width(), screen_height());
    draw_rectangle(0f32, 0f32, win_w, win_h, color_u8!(0, 0, 0, 255));
    let mut y = 0f32;
    HELP_PAGE.lines().enumerate().for_each(|(i, l)| {
        y = (i * ctx.font_size as usize) as f32;
        draw_text_ex(
            l,
            0f32,
            y,
            TextParams {
                font_size: ctx.font_size,
                color: color_u8!(255, 255, 255, 255),
                font: ctx.font,
                ..Default::default()
            },
        );
    });
}
