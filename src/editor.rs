use std::str::FromStr;

use macroquad::{
    input,
    prelude::{
        is_key_down, is_key_pressed, is_mouse_button_pressed, is_quit_requested, KeyCode,
        MouseButton,
    },
    window::screen_height,
};

use crate::{
    core::{Context, Modes, SearchResults},
    render::{from_cells_to_string, from_str_to_cells, render},
};

pub enum Command {
    Exit,
    PageUp,
    PageDown,
    GoTop,
    GoBottom,
    GoToLine,
    FindCaseSensitive,
    FindInCaseSensitive,
    Home,
    Help,
    End,
    Save,
    MoveLeft,
    OpenDocument,
    MoveRight,
    MoveUp,
    MoveDown,
    Delete,
    Enter,
    Backspace,
    WordMoveLeft,
    WordMoveRight,
    MouseLeftClick,
    CharPressed(char),
    DeleteWord,
}

pub fn get_command() -> Option<Command> {
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
        Some(Command::Save)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Left) {
        Some(Command::WordMoveLeft)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::W) {
        Some(Command::DeleteWord)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::H) {
        Some(Command::Help)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Right) {
        Some(Command::WordMoveRight)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::G) {
        Some(Command::GoToLine)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::O) {
        Some(Command::OpenDocument)
    } else if is_key_down(KeyCode::LeftControl)
        && !is_key_down(KeyCode::LeftShift)
        && is_key_pressed(KeyCode::F)
    {
        Some(Command::FindCaseSensitive)
    } else if is_key_down(KeyCode::LeftControl)
        && is_key_down(KeyCode::LeftShift)
        && is_key_pressed(KeyCode::F)
    {
        Some(Command::FindInCaseSensitive)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::PageUp) {
        Some(Command::GoTop)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::PageDown) {
        Some(Command::GoBottom)
    } else if (is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Q))
        || is_quit_requested()
    {
        Some(Command::Exit)
    } else if is_key_pressed(KeyCode::PageUp) {
        Some(Command::PageUp)
    } else if is_key_pressed(KeyCode::Enter) {
        Some(Command::Enter)
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
    } else if is_key_pressed(KeyCode::Backspace) {
        Some(Command::Backspace)
    } else if !is_key_down(KeyCode::LeftControl) && input::get_last_key_pressed().is_some() {
        if let Some(c) = input::get_char_pressed() {
            if !(c.is_alphanumeric()
                || c == ' '
                || c == '\t'
                || c.is_ascii_punctuation()
                || c.is_ascii_graphic())
            {
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
    } else {
        None
    }
}

fn update_view_buffer(ctx: &mut Context) {
    ctx.vert_cell_count.1 = screen_height() as usize / ctx.font_size as usize + 1;
    from_str_to_cells(ctx);
}

pub async fn go_to_line(ctx: &mut Context) -> usize {
    ctx.prompt_input.clear();
    loop {
        if let Some(key) = input::get_last_key_pressed() {
            if key == KeyCode::Escape {
                ctx.prompt_input.clear();
                return ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 + 1;
            }
            if key == KeyCode::Enter {
                break;
            }
            if key == KeyCode::Backspace {
                ctx.prompt_input.pop();
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

pub async fn find_in_buf(ctx: &mut Context, is_case_sensitive: bool) -> (usize, usize) {
    ctx.prompt_input = ctx.last_searched_term.clone();
    let _ = input::get_char_pressed();
    loop {
        if let Some(key) = input::get_last_key_pressed() {
            if key == KeyCode::Escape {
                ctx.is_search_changed = false;
                ctx.prompt_input.clear();
                return (
                    ctx.curr_cursor_pos.0,
                    ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1,
                );
            }
            if key == KeyCode::Backspace {
                ctx.prompt_input.pop();
                ctx.last_searched_term = ctx.prompt_input.clone();
            }
            if key == KeyCode::Enter {
                let _ = input::get_char_pressed();
                if ctx.is_search_changed {
                    let mut searchidx: usize = 0;
                    let mut searchres: SearchResults = Default::default();
                    for (i, l) in ctx.buffer.buf.iter().enumerate() {
                        if is_case_sensitive == false {
                            let s = l.to_ascii_lowercase();
                            let p = ctx.prompt_input.to_ascii_lowercase();
                            for (idx, _) in s.match_indices(&p) {
                                searchres.push((searchidx, (idx, i)));
                                searchidx = searchidx.saturating_add(1);
                            }
                        } else {
                            for (idx, _) in l.match_indices(&ctx.prompt_input) {
                                searchres.push((searchidx, (idx, i)));
                                searchidx = searchidx.saturating_add(1);
                            }
                        }
                    }
                    ctx.search_res = searchres;
                    if ctx.last_searched_idx >= ctx.search_res.len() {
                        ctx.last_searched_idx = 0;
                    }
                } else {
                    if ctx.search_res.is_empty() {
                        return (
                            ctx.curr_cursor_pos.0,
                            ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1,
                        );
                    }
                    if is_key_down(KeyCode::LeftShift) {
                        if (ctx.last_searched_idx as isize - 1) >= 0 {
                            ctx.last_searched_idx -= 1;
                        } else {
                            ctx.last_searched_idx = ctx.search_res.len() - 1;
                        }
                    } else {
                        if (ctx.last_searched_idx + 1) < ctx.search_res.len() {
                            ctx.last_searched_idx += 1;
                        } else {
                            ctx.last_searched_idx = 0;
                        }
                    }
                    let pos = ctx
                        .search_res
                        .iter()
                        .find(|&&(idx, _)| idx == ctx.last_searched_idx)
                        .unwrap()
                        .1;
                    ctx.vert_cell_count.0 = pos.1;
                    ctx.curr_cursor_pos = (pos.0, 0);
                }
                ctx.is_search_changed = false;
                update_view_buffer(ctx);
            }
            if let Some(c) = input::get_char_pressed() {
                if c.is_ascii_graphic() || c.is_ascii_whitespace() {
                    ctx.is_search_changed = true;
                    ctx.prompt_input.push(c);
                    ctx.last_searched_term = ctx.prompt_input.clone();
                }
            }
        }
        render(ctx).await;
    }
}

pub async fn show_help_page(ctx: &mut Context) {
    loop {
        if let Some(k) = input::get_last_key_pressed() {
            if k == KeyCode::Escape {
                return;
            }
        }
        render(ctx).await;
    }
}

pub async fn prompt_unsaved_changes(ctx: &mut Context) {
    loop {
        if let Some(c) = input::get_char_pressed() {
            if c == 'y' {
                ctx.is_exit = true;
                return;
            } else if c == 'n' {
                return;
            }
        }
        render(ctx).await;
    }
}

fn move_cursor_up(ctx: &mut Context) {
    if ctx.vert_cell_count.0 == 0 && ctx.curr_cursor_pos.1 == 0 {
        return ();
    }
    if ctx.curr_cursor_pos.1 == 0 && ctx.buffer.buf.get(ctx.vert_cell_count.0 - 1).is_some() {
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
        .find(|c| (c.pos.1 == ctx.curr_cursor_pos.1 - 1) && (c.pos.0 == ctx.curr_cursor_pos.0))
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

fn move_cursor_down(ctx: &mut Context) {
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
        .find(|c| (c.pos.1 == ctx.curr_cursor_pos.1 + 1) && (c.pos.0 == ctx.curr_cursor_pos.0))
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

fn move_cursor_left(ctx: &mut Context) {
    if ctx.curr_cursor_pos.0 == 0 {
        ()
    } else {
        ctx.curr_cursor_pos.0 -= 1
    }
    dbg!(ctx.curr_cursor_pos);
}

fn move_cursor_right(ctx: &mut Context) {
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

fn move_cursor_left_word(ctx: &mut Context) {
    let view_buffer = from_cells_to_string(&ctx.cells);
    let row = &view_buffer[ctx.curr_cursor_pos.1];
    let sub = &row[..ctx.curr_cursor_pos.0];
    if let Some(space_indx) = sub.rfind(' ') {
        ctx.curr_cursor_pos.0 -= ctx.curr_cursor_pos.0 - space_indx;
    }
}

fn move_cursor_right_word(ctx: &mut Context) {
    let view_buffer = from_cells_to_string(&ctx.cells);
    let row = &view_buffer[ctx.curr_cursor_pos.1];
    let sub = &row[ctx.curr_cursor_pos.0..];
    if let Some(space_indx) = sub.find(' ') {
        ctx.curr_cursor_pos.0 += space_indx + 1;
    }
}

fn delete_word(ctx: &mut Context) {
    let str =
        &ctx.buffer.buf[ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1][..ctx.curr_cursor_pos.0];
    dbg!(str);
    if let Some(idx) = str.rfind(' ') {
        ctx.buffer.buf[ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1]
            .replace_range(idx..ctx.curr_cursor_pos.0, "");
        ctx.curr_cursor_pos.0 = idx;
    }
    update_view_buffer(ctx);
}

pub async fn update_state(ctx: &mut Context) {
    match get_command() {
        Some(Command::OpenDocument) => {
            todo!() // TODO
        }
        Some(Command::DeleteWord) => {
            delete_word(ctx);
        }
        Some(Command::Enter) => {
            if ctx.mode == Modes::Edit {
                ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                    if ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 == i {
                        s.insert(ctx.curr_cursor_pos.0, '\n');
                    }
                });
                let idx = ctx.buffer.buf[ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1]
                    .find('\n')
                    .unwrap();
                let s = String::from_str(
                    &ctx.buffer.buf[ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1][idx + 1..],
                )
                .unwrap();
                ctx.buffer
                    .buf
                    .insert(ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 + 1, s);
                ctx.buffer.buf[ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1]
                    .replace_range(idx + 1.., "");
                move_cursor_down(ctx);
                ctx.curr_cursor_pos.0 = 0;
                update_view_buffer(ctx);
            }
        }
        Some(Command::Exit) => {
            if !ctx.is_file_changed {
                ctx.is_exit = true
            } else {
                ctx.mode = Modes::ModifiedPrompt;
                prompt_unsaved_changes(ctx).await;
                ctx.mode = Modes::Edit;
            }
        }
        Some(Command::Save) => {
            if ctx.is_file_changed {
                ctx.buffer.write_to_file();
            }
            ctx.is_file_changed = false;
        }
        Some(Command::Help) => {
            ctx.mode = Modes::ShowHelp;
            show_help_page(ctx).await;
            ctx.mode = Modes::Edit;
        }
        Some(Command::FindInCaseSensitive) => {
            ctx.mode = Modes::FindCaseInSensitive;
            let search_pos = find_in_buf(ctx, false).await;
            ctx.vert_cell_count.0 = search_pos.1;
            ctx.curr_cursor_pos = (search_pos.0, 0);
            update_view_buffer(ctx);
            ctx.prompt_input.clear();
            ctx.mode = Modes::Edit;
        }
        Some(Command::FindCaseSensitive) => {
            ctx.mode = Modes::FindCaseSensitive;
            let search_pos = find_in_buf(ctx, true).await;
            ctx.vert_cell_count.0 = search_pos.1;
            ctx.curr_cursor_pos = (search_pos.0, 0);
            update_view_buffer(ctx);
            ctx.prompt_input.clear();
            ctx.mode = Modes::Edit;
        }
        Some(Command::GoToLine) => {
            ctx.mode = Modes::GoToLine;
            let line = go_to_line(ctx).await;
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
            let cell = ctx
                .cells
                .iter()
                .filter(|c| c.pos.1 == cell_y)
                .find(|c| c.coord.0 < x && x < (c.coord.0 + c.bound.0));
            if let Some(c) = cell {
                ctx.curr_cursor_pos.0 = c.pos.0;
                ctx.curr_cursor_pos.1 = cell_y;
            } else {
                let cel = ctx
                    .cells
                    .iter()
                    .filter(|c| c.pos.1 == cell_y)
                    .find(|c| c.c == '\n');
                if cel.is_some() {
                    ctx.curr_cursor_pos = (cel.unwrap().pos.0, cell_y);
                } else {
                    ctx.curr_cursor_pos = (0, 0);
                }
            }
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
            move_cursor_right_word(ctx);
        }
        Some(Command::WordMoveLeft) => {
            move_cursor_left_word(ctx);
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
            move_cursor_up(ctx);
        }
        Some(Command::MoveDown) => {
            move_cursor_down(ctx);
        }
        Some(Command::MoveLeft) => {
            move_cursor_left(ctx);
        }
        Some(Command::MoveRight) => {
            move_cursor_right(ctx);
        }
        // TODO: implement Enter, Backspace, Delete key
        Some(Command::Backspace) => {
            ctx.is_file_changed = true;
            update_view_buffer(ctx);
        }
        Some(Command::Delete) => {
            ctx.is_file_changed = true;
            ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                if ctx.curr_cursor_pos.1 == i {
                    s.remove(ctx.curr_cursor_pos.0);
                }
            });
            update_view_buffer(ctx);
        }
        Some(Command::CharPressed(c)) => {
            ctx.is_file_changed = true;
            ctx.buffer.buf.iter_mut().enumerate().for_each(|(i, s)| {
                if ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1 == i {
                    if c == '\t' {
                        for _ in 0..ctx.tab_width {
                            s.insert(ctx.curr_cursor_pos.0, ' ');
                            ctx.curr_cursor_pos.0 += 1;
                        }
                    } else {
                        s.insert(ctx.curr_cursor_pos.0, c);
                        ctx.curr_cursor_pos.0 += 1;
                    }
                }
            });
            update_view_buffer(ctx);
        }
        None => (),
    }
}
