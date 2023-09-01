use macroquad::{
    input,
    prelude::{
        is_key_down, is_key_pressed, is_mouse_button_pressed, is_quit_requested, KeyCode,
        MouseButton,
    },
};

use crate::{
    core::{Context, Modes},
    render::{from_cells_to_string, from_str_to_cells, render},
};

pub enum Command {
    Exit,
    PageUp,
    PageDown,
    GoTop,
    GoBottom,
    GoToLine,
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
    MouseLeftClick,
    CharPressed(char),
}

pub fn get_input() -> Option<Command> {
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
        Some(Command::Save)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Left) {
        Some(Command::WordMoveLeft)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Right) {
        Some(Command::WordMoveRight)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::G) {
        Some(Command::GoToLine)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::PageUp) {
        Some(Command::GoTop)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::PageDown) {
        Some(Command::GoBottom)
    } else if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
        Some(Command::Exit)
    } else if is_key_pressed(KeyCode::PageUp) {
        Some(Command::PageUp)
    } else if is_mouse_button_pressed(MouseButton::Left) {
        Some(Command::MouseLeftClick)
    } else if is_key_pressed(KeyCode::PageDown) {
        Some(Command::PageDown)
    } else if is_key_pressed(KeyCode::Home) {
        Some(Command::Home)
    } else if is_key_pressed(KeyCode::End) {
        Some(Command::End)
    } else if is_key_pressed(KeyCode::Up) {
        Some(Command::MoveUp)
    } else if is_key_pressed(KeyCode::Down) {
        Some(Command::MoveDown)
    } else if is_key_pressed(KeyCode::Left) {
        Some(Command::MoveLeft)
    } else if is_key_pressed(KeyCode::Right) {
        Some(Command::MoveRight)
    } else if is_key_pressed(KeyCode::Delete) {
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

fn update_view_buffer(ctx: &mut Context) {
    ctx.vert_cell_count.1 =
        macroquad::window::screen_height() as usize / ctx.font_size as usize + 1;
    from_str_to_cells(ctx);
}

pub async fn handle_go_to_line(ctx: &mut Context) -> usize {
    loop {
        if let Some(key) = input::get_last_key_pressed() {
            if key == KeyCode::Escape {
                ctx.prompt_input.clear();
                return ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 + 1;
            }
            if key == KeyCode::Enter {
                break;
            }
            if let Some(c) = input::get_char_pressed() {
                if c.is_ascii_digit() {
                    ctx.prompt_input.push(c);
                }
            }
        }
        render(ctx).await;
    }
    return ctx
        .prompt_input
        .parse::<usize>()
        .unwrap_or(ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 + 1);
}

pub async fn update_state(ctx: &mut Context) {
    match get_input() {
        Some(Command::Exit) => ctx.is_exit = true,
        Some(Command::Save) => ctx.buffer.write_to_file(),
        Some(Command::GoToLine) => {
            ctx.mode = Modes::GoToLine;
            let line = handle_go_to_line(ctx).await;
            let line = std::cmp::max(std::cmp::min(ctx.buffer.buf.len(), line), 1);
            ctx.vert_cell_count.0 = line - 1;
            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
            ctx.prompt_input.clear();
            ctx.mode = Modes::Edit;
        }
        Some(Command::MouseLeftClick) => {
            let (x, y) = input::mouse_position();
            let cell_y = (y / ctx.font_size as f32).floor() as usize;
            let cell = ctx.cells.iter().filter(|c| c.pos.1 == cell_y).find(|c| c.coord.0 < x && x < (c.coord.0 + c.bound.0));
            if let Some(c) = cell {
                ctx.curr_cursor_pos.0 = c.pos.0;
            } else {
                ctx.curr_cursor_pos.0 = ctx.cells.iter().filter(|c| c.pos.1 == cell_y).find(|c| c.c == '\n').unwrap().pos.0;
            }
            ctx.curr_cursor_pos.1 = cell_y;
        }
        Some(Command::GoTop) => {
            update_view_buffer(ctx);
            ctx.vert_cell_count.0 = 0;
            ctx.curr_cursor_pos = (0, 0);
            from_str_to_cells(ctx);
        }
        Some(Command::GoBottom) => {
            update_view_buffer(ctx);
            ctx.vert_cell_count.0 = ctx.buffer.buf.len() - 1;
            ctx.curr_cursor_pos = (0, 0);
            from_str_to_cells(ctx);
        }
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
                ctx.curr_cursor_pos.0 -= ctx.curr_cursor_pos.0 - space_indx;
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
            update_view_buffer(ctx);
            ctx.vert_cell_count.0 = std::cmp::max(
                ctx.vert_cell_count
                    .0
                    .saturating_sub(ctx.vert_cell_count.1 as usize),
                0,
            );

            ctx.curr_cursor_pos = (0, 0);
            from_str_to_cells(ctx);
            // dbg!(from_cells_to_string(&ctx.cells));
        }
        Some(Command::PageDown) => {
            update_view_buffer(ctx);
            ctx.vert_cell_count.0 = std::cmp::min(
                ctx.vert_cell_count
                    .0
                    .saturating_add(ctx.vert_cell_count.1 as usize),
                ctx.buffer.buf.len(),
            ) - 1;

            ctx.curr_cursor_pos = (0, 0);
            from_str_to_cells(ctx);
            // dbg!(from_cells_to_string(&ctx.cells));
        }
        Some(Command::MoveUp) => {
            if ctx.vert_cell_count.0 == 0 && ctx.curr_cursor_pos.1 == 0 {
                return ();
            }
            if ctx.curr_cursor_pos.1 == 0 && ctx.buffer.buf.get(ctx.vert_cell_count.0 - 1).is_some()
            {
                update_view_buffer(ctx);
                ctx.vert_cell_count.0 -= 1;
                ctx.curr_cursor_pos = (0, 0);
                from_str_to_cells(ctx);
                return ();
            }
            update_view_buffer(ctx);
            let view_buffer = from_cells_to_string(&ctx.cells);
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
                ctx.curr_cursor_pos.0 = view_buffer[ctx.curr_cursor_pos.1 - 1]
                    .char_indices()
                    .last()
                    .unwrap()
                    .0;
                ctx.curr_cursor_pos.1 -= 1;
            }
            dbg!(ctx.curr_cursor_pos);
        }
        Some(Command::MoveDown) => {
            if ctx.vert_cell_count.0 == ctx.buffer.buf.len() - 1
                || (ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1) == (ctx.buffer.buf.len() - 1)
            {
                return ();
            }
            if ctx.curr_cursor_pos.1 == (ctx.vert_cell_count.1 - 1)
                && ctx
                    .buffer
                    .buf
                    .get(ctx.vert_cell_count.0 + ctx.vert_cell_count.1)
                    .is_some()
            {
                update_view_buffer(ctx);
                ctx.vert_cell_count.0 += 1;
                ctx.curr_cursor_pos = (0, ctx.vert_cell_count.1 - 2);
                from_str_to_cells(ctx);
                return ();
            }
            update_view_buffer(ctx);
            let view_buffer = from_cells_to_string(&ctx.cells);
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
                if view_buffer.get(ctx.curr_cursor_pos.1 + 1).is_none() {
                    return;
                }
                ctx.curr_cursor_pos.0 = view_buffer[ctx.curr_cursor_pos.1 + 1]
                    .char_indices()
                    .last()
                    .unwrap()
                    .0;
                ctx.curr_cursor_pos.1 += 1
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
            let view_buffer = from_cells_to_string(&ctx.cells);
            if view_buffer[ctx.curr_cursor_pos.1]
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
        // TODO: apply all movements the view_buffer
        Some(Command::Delete) => ()/*ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
            if ctx.curr_cursor_pos.1 == i {
                s.remove(ctx.curr_cursor_pos.0);
            }
        })*/,
        Some(Command::CharPressed(_c)) => ()/*{
            ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                if ctx.curr_cursor_pos.1 == i {
                    s.insert(ctx.curr_cursor_pos.0, c);
                }
            })
        }*/,
        None => (),
    }
}
