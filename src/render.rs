use macroquad::{prelude::*, window};

use crate::core::Context;

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
        ctx.buffer.buf.len(),
    );
    dbg!(ctx.vert_cell_count.0);
    dbg!(end_rng);

    for (i, s) in ctx.buffer.buf[ctx.vert_cell_count.0..end_rng]
        .iter_mut()
        .enumerate()
    {
        for (j, ch) in s.chars().enumerate() {
            let letter_size = measure_text(&ch.to_string(), Some(ctx.font), ctx.font_size, 1.0f32);
            x_coor = measure_text(&s[..j], Some(ctx.font), ctx.font_size, 1f32).width;
            cells.push(Cell {
                c: ch,
                coord: (x_coor, y_coor + 14f32),
                bound: (letter_size.width, ctx.font_size as f32),
                pos: (j, i),
            });
        }
        y_coor = (i + 1) as f32 * ctx.font_size as f32;
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
        "{}:{}:{}",
        ctx.active_buf.display(),
        ctx.vert_cell_count.0 + 1,
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
        14f32,
        TextParams {
            font_size: ctx.font_size,
            color: color_u8!(0, 0, 0, 255),
            font: ctx.font,
            ..Default::default()
        },
    );
}

pub async fn render(ctx: &Context) {
    clear_background(ctx.bg_color);
    for cell in ctx.cells.iter() {
        if cell.c == '\n' {
            continue;
        }
        draw_text_ex(
            &cell.c.to_string(),
            cell.coord.0,
            cell.coord.1,
            TextParams {
                font_size: ctx.font_size,
                color: ctx.font_color,
                font: ctx.font,
                ..Default::default()
            },
        );
    }
    let cursor_to_render = ctx
        .cells
        .iter()
        .filter(|c| c.pos == ctx.curr_cursor_pos)
        .next()
        .unwrap();
    draw_rectangle(
        cursor_to_render.coord.0,
        cursor_to_render.coord.1 - 12f32,
        cursor_to_render.bound.0,
        cursor_to_render.bound.1,
        ctx.cursor_col,
    );
    draw_cursor_location(ctx);
    next_frame().await
}
