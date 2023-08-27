use macroquad::{
    input,
    prelude::{is_key_down, is_key_pressed, is_quit_requested, screen_height, KeyCode},
};

use crate::{core::Context, render::{from_str_to_cells, from_cells_to_string}};

pub enum Command {
    Exit,
    PageUp,
    PageDown,
    Home,
    End,
    Save,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Delete,
    WordMoveLeft,
    WordMoveRight,
    CharPressed(char),
}

pub fn get_input() -> Option<Command> {
    if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::S) {
        Some(Command::Save)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Left) {
        Some(Command::WordMoveLeft)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Right) {
        Some(Command::WordMoveRight)
    } else if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
        Some(Command::Exit)
    } else if is_key_pressed(KeyCode::Up) {
        Some(Command::MoveUp)
    } else if is_key_pressed(KeyCode::PageUp) {
        Some(Command::PageUp)
    } else if is_key_pressed(KeyCode::PageDown) {
        Some(Command::PageDown)
    } else if is_key_pressed(KeyCode::Home) {
        Some(Command::Home)
    } else if is_key_pressed(KeyCode::End) {
        Some(Command::End)
    } else if is_key_pressed(KeyCode::Down) {
        Some(Command::MoveDown)
    } else if is_key_pressed(KeyCode::Left) {
        Some(Command::MoveLeft)
    } else if is_key_pressed(KeyCode::Right) {
        Some(Command::MoveRight)
    } else if is_key_pressed(KeyCode::Backspace) {
        Some(Command::Delete)
    } else {
        if let Some(c) = input::get_char_pressed() {
            if !c.is_alphanumeric() {
                return None;
            }
            if is_key_down(KeyCode::LeftShift) {
                Some(Command::CharPressed(c.to_ascii_uppercase()))
            } else {
                Some(Command::CharPressed(c))
            }
        } else {
            None
        }
    }
}

pub fn update_state(ctx: &mut Context) {
    ctx.vert_cell_count.1 = screen_height() as usize / ctx.font_size as usize + 1;

    match get_input() {
        Some(Command::Exit) => ctx.is_exit = true,
        Some(Command::Save) => ctx.buffer.write_to_file(),
        // TODO: apply all movements the view_buffer
        Some(Command::WordMoveRight) => {
            let view_buffer = from_cells_to_string(&ctx.cells);
            let row = &view_buffer[ctx.curr_cursor_pos.1];
            let sub = &row[ctx.curr_cursor_pos.0..];
            if let Some(space_indx) = sub.find(' ') {
                ctx.curr_cursor_pos.0 += space_indx + 1;
            }
        }
        Some(Command::WordMoveLeft) => {
            let view_buffer = from_cells_to_string(&ctx.cells);
            let row = &view_buffer[ctx.curr_cursor_pos.1];
            let sub = &row[..ctx.curr_cursor_pos.0];
            if let Some(space_indx) = sub.rfind(' ') {
                ctx.curr_cursor_pos.0 -= ctx.curr_cursor_pos.0 - space_indx + 1;
            }
        }
        Some(Command::Home) => ctx.curr_cursor_pos.0 = 0,
        Some(Command::End) => {
            let view_buffer = from_cells_to_string(&ctx.cells);
            ctx.curr_cursor_pos.0 = view_buffer[ctx.curr_cursor_pos.1]
                .char_indices()
                .last()
                .unwrap()
                .0;
        }
        Some(Command::PageUp) => {
            ctx.vert_cell_count.0 =
                std::cmp::max(ctx.vert_cell_count.0 - ctx.vert_cell_count.1 as usize, 0);

            ctx.curr_cursor_pos.0 = 0;
            ctx.curr_cursor_pos.1 = ctx.vert_cell_count.0 % ctx.vert_cell_count.1;
            from_str_to_cells(ctx);
        }
        Some(Command::PageDown) => {
            ctx.vert_cell_count.0 = std::cmp::min(
                ctx.vert_cell_count.0 + ctx.vert_cell_count.1 as usize,
                ctx.buffer.buf.len() - 1,
            );

            ctx.curr_cursor_pos.0 = 0;
            ctx.curr_cursor_pos.1 = ctx.vert_cell_count.0 % ctx.vert_cell_count.1;
            from_str_to_cells(ctx);
        }
        Some(Command::MoveUp) => {
            if ctx.curr_cursor_pos.1 == 0 {
                ();
            } else {
                if ctx
                    .cells
                    .iter()
                    .find(|c| {
                        (c.pos.1 == ctx.curr_cursor_pos.1 - 1) && (c.pos.0 == ctx.curr_cursor_pos.0)
                    })
                    .is_some()
                {
                    ctx.curr_cursor_pos.1 -= 1;
                } else {
                    ctx.curr_cursor_pos.0 = ctx.buffer.buf[ctx.curr_cursor_pos.1 - 1]
                        .char_indices()
                        .last()
                        .unwrap()
                        .0;
                    ctx.curr_cursor_pos.1 -= 1;
                }
            }
            dbg!(ctx.curr_cursor_pos);
        }
        Some(Command::MoveDown) => {
            if ctx.curr_cursor_pos.1 == ctx.buffer.buf.len() - 1 {
                ()
            } else {
                if ctx
                    .cells
                    .iter()
                    .find(|c| {
                        (c.pos.1 == ctx.curr_cursor_pos.1 + 1) && (c.pos.0 == ctx.curr_cursor_pos.0)
                    })
                    .is_some()
                {
                    ctx.curr_cursor_pos.1 += 1
                } else {
                    ctx.curr_cursor_pos.0 = ctx.buffer.buf[ctx.curr_cursor_pos.1 + 1]
                        .char_indices()
                        .last()
                        .unwrap()
                        .0;
                    ctx.curr_cursor_pos.1 += 1
                }
            }
            dbg!(ctx.curr_cursor_pos);
        }
        Some(Command::MoveLeft) => {
            if ctx.curr_cursor_pos.0 == 0 {
                ()
            } else {
                ctx.curr_cursor_pos.0 -= 1
            }
            dbg!(ctx.curr_cursor_pos);
        }
        Some(Command::MoveRight) => {
            if ctx.buffer.buf[ctx.curr_cursor_pos.1]
                .chars()
                .nth(ctx.curr_cursor_pos.0)
                .unwrap()
                == '\n'
            {
                ()
            } else {
                ctx.curr_cursor_pos.0 += 1
            }
            dbg!(ctx.curr_cursor_pos);
        }
        Some(Command::Delete) => ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
            if ctx.curr_cursor_pos.1 == i {
                s.remove(ctx.curr_cursor_pos.0);
            }
        }),
        Some(Command::CharPressed(c)) => {
            ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                if ctx.curr_cursor_pos.1 == i {
                    s.insert(ctx.curr_cursor_pos.0, c);
                }
            })
        }
        None => (),
    }
}
