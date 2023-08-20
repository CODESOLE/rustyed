use macroquad::{
    input,
    prelude::{is_key_down, is_key_pressed, is_quit_requested, KeyCode},
};

use crate::core::Context;

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
    } else if input::get_char_pressed().is_some() {
        if is_key_down(KeyCode::LeftShift) {
            Some(Command::CharPressed(
                input::get_char_pressed().unwrap().to_ascii_uppercase(),
            ))
        } else {
            Some(Command::CharPressed(input::get_char_pressed().unwrap()))
        }
    } else {
        None
    }
}

pub fn update_state(ctx: &mut Context) {
    match get_input() {
        Some(Command::Exit) => ctx.is_exit = true,
        Some(Command::Save) => ctx.buffer.write_to_file(),
        Some(Command::WordMoveRight) => {
            let row = &ctx.buffer.buf[ctx.curr_cursor_pos.1];
            let sub = &row[ctx.curr_cursor_pos.0..];
            if let Some(space_indx) = sub.find(' ') {
                ctx.curr_cursor_pos.0 += space_indx + 1
            }
        }
        Some(Command::WordMoveLeft) => {
            let row = &ctx.buffer.buf[ctx.curr_cursor_pos.1];
            let sub = &row[..ctx.curr_cursor_pos.0];
            if let Some(space_indx) = sub.rfind(' ') {
                ctx.curr_cursor_pos.0 -= ctx.curr_cursor_pos.0 - space_indx + 1
            }
        }
        Some(Command::Home) => ctx.curr_cursor_pos.0 = 0,
        Some(Command::End) => {
            ctx.curr_cursor_pos.0 = ctx.buffer.buf[ctx.curr_cursor_pos.1]
                .char_indices()
                .last()
                .unwrap()
                .0
        }
        Some(Command::PageUp) => {
            ctx.curr_cursor_pos.0 = 0;
            ctx.curr_cursor_pos.1 = 0
        }
        Some(Command::PageDown) => {
            ctx.curr_cursor_pos.0 = 0;
            ctx.curr_cursor_pos.1 = ctx.buffer.buf.len() - 1
        }
        Some(Command::MoveUp) => {
            if ctx.curr_cursor_pos.1 == 0 {
                ()
            } else {
                if ctx
                    .cells
                    .iter()
                    .find(|c| {
                        (c.pos.1 == ctx.curr_cursor_pos.1 - 1) && (c.pos.0 == ctx.curr_cursor_pos.0)
                    })
                    .is_some()
                {
                    ctx.curr_cursor_pos.1 -= 1
                } else {
                    ctx.curr_cursor_pos.0 = ctx.buffer.buf[ctx.curr_cursor_pos.1 - 1]
                        .char_indices()
                        .last()
                        .unwrap()
                        .0;
                    ctx.curr_cursor_pos.1 -= 1
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
            if ctx.curr_cursor_pos.1 as usize == i {
                s.remove(ctx.curr_cursor_pos.0 as usize);
            }
        }),
        Some(Command::CharPressed(c)) => {
            if !c.is_alphanumeric() {
                return ();
            }
            ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                if ctx.curr_cursor_pos.1 as usize == i {
                    s.insert(ctx.curr_cursor_pos.0 as usize, c);
                }
            })
        }
        None => (),
    }
}
