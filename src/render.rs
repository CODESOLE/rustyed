use macroquad::prelude::*;

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
    let mut y_coor;

    for (i, s) in ctx.buffer.buf.iter().enumerate() {
        y_coor = i as f32 * ctx.font_size as f32;
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
    }
    ctx.cells = cells;
}

pub async fn render(ctx: &Context) {
    clear_background(ctx.bg_color);
    for _ in ctx.buffer.buf.iter() {
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
    next_frame().await
}
