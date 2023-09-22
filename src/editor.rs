use std::str::FromStr;

use macroquad::{
    input,
    prelude::{
        is_key_down, is_key_pressed, is_mouse_button_pressed, is_quit_requested, KeyCode,
        MouseButton,
    },
    window::screen_height,
};
use rfd::FileDialog;
use undo::Record;

use crate::{
    core::{Context, Modes, SearchResults},
    render::{from_cells_to_string, from_str_to_cells, render},
};

pub enum Command {
    Exit,
    PageUp,
    PageDown,
    ShiftPageUp,
    ShiftPageDown,
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
    WordMoveLeft,
    WordMoveRight,
    MouseLeftClick,
    ShiftSelectUp,
    ShiftSelectDown,
    ShiftSelectLeft,
    ShiftSelectRight,
    Undo,
    Redo,
    Copy,
    Paste,
    Cut,
    Delete,
    Enter,
    Backspace,
    CharPressed(char),
    DeleteWord,
}

pub fn get_command() -> Option<Command> {
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
        Some(Command::Save)
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Left) {
        Some(Command::ShiftSelectLeft)
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Right) {
        Some(Command::ShiftSelectRight)
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Up) {
        Some(Command::ShiftSelectUp)
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Down) {
        Some(Command::ShiftSelectDown)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::C) {
        Some(Command::Copy)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::X) {
        Some(Command::Cut)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::V) {
        Some(Command::Paste)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Z) {
        Some(Command::Undo)
    } else if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Y) {
        Some(Command::Redo)
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
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::PageUp) {
        Some(Command::ShiftPageUp)
    } else if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::PageDown) {
        Some(Command::ShiftPageDown)
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
                    for (i, l) in ctx.buffer.buf.lines().enumerate() {
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
    let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
    if ctx.curr_cursor_pos.1 == 0 && inter_buf_off.0.is_some() {
        ctx.vert_cell_count.0 -= 1;
        ctx.curr_cursor_pos = (0, 0);
        update_view_buffer(ctx);
        return ();
    }
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
    update_view_buffer(ctx);
}

fn move_cursor_down(ctx: &mut Context) {
    if ctx.vert_cell_count.0 == ctx.buffer.buf.lines().count() - 1
        || (ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1) == (ctx.buffer.buf.lines().count() - 1)
    {
        return ();
    }
    let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
    if ctx.curr_cursor_pos.1 == (ctx.vert_cell_count.1 - 1)
        && ctx.buffer.buf[inter_buf_off.1..]
            .chars()
            .find(|&c| c == '\n')
            .is_some()
    {
        ctx.vert_cell_count.0 += 1;
        ctx.curr_cursor_pos = (0, ctx.vert_cell_count.1 - 2);
        update_view_buffer(ctx);
        return ();
    }
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
    update_view_buffer(ctx);
}

fn move_cursor_left(ctx: &mut Context) {
    if ctx.curr_cursor_pos.0 == 0 {
        ()
    } else {
        ctx.curr_cursor_pos.0 -= 1
    }
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

#[derive(Debug)]
pub struct InternalBufOffset(Option<usize>, usize);
impl InternalBufOffset {
    pub fn get_inline_offset(&self) -> usize {
        if self.0.is_some() {
            self.1 - self.0.unwrap() - 1
        } else {
            self.1
        }
    }
}

fn get_internal_buf_offset(ctx: &Context) -> Option<InternalBufOffset> {
    if ctx.vert_cell_count.0 == 0 && ctx.curr_cursor_pos.1 == 0 {
        return Some(InternalBufOffset(None, ctx.curr_cursor_pos.0));
    }
    ctx.buffer
        .buf
        .match_indices('\n')
        .nth((ctx.vert_cell_count.0 + ctx.curr_cursor_pos.1).saturating_sub(1))
        .map(|(index, _)| InternalBufOffset(Some(index), index + ctx.curr_cursor_pos.0 + 1))
}

pub fn get_ch_off_to_inline_off(ctx: &Context, off: usize) -> usize {
    let mut offset = 0usize;
    for i in (0..off).rev() {
        if ctx.buffer.buf.chars().nth(i).unwrap() != '\n' {
            offset += 1;
        } else {
            return offset;
        }
    }
    offset
}

fn delete_selection(ctx: &mut Context, record: &mut Record<Change>) {
    if ctx.selection_range.unwrap().0 .0 < ctx.selection_range.unwrap().1 .0 {
        ctx.curr_cursor_pos = ctx.selection_range.unwrap().0 .1;
        record.apply(
            ctx,
            Change::DeleteSelection(
                ctx.selection_range.unwrap().0 .0,
                String::from_str(
                    &ctx.buffer.buf
                        [ctx.selection_range.unwrap().0 .0..=ctx.selection_range.unwrap().1 .0],
                )
                .unwrap(),
            ),
        );
    } else {
        ctx.curr_cursor_pos = ctx.selection_range.unwrap().1 .1;
        record.apply(
            ctx,
            Change::DeleteSelection(
                ctx.selection_range.unwrap().1 .0,
                String::from_str(
                    &ctx.buffer.buf
                        [ctx.selection_range.unwrap().1 .0..=ctx.selection_range.unwrap().0 .0],
                )
                .unwrap(),
            ),
        );
    }
    ctx.selection_range = None;
}

fn delete_word(ctx: &mut Context, record: &mut Record<Change>) {
    let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
    let inline_offset = inter_buf_off.get_inline_offset();
    let str;
    if let Some(lfidx) = inter_buf_off.0 {
        str = &ctx.buffer.buf[lfidx + 1..inter_buf_off.1];
    } else {
        str = &ctx.buffer.buf[..inter_buf_off.1];
    }
    if let Some(idx) = str.rfind(' ') {
        record.apply(
            ctx,
            Change::DeleteWord(
                inter_buf_off.1 - (inline_offset - idx),
                String::from_str(
                    &ctx.buffer.buf[inter_buf_off.1 - (inline_offset - idx)..inter_buf_off.1],
                )
                .unwrap(),
            ),
        );
        // ctx.buffer
        //     .buf
        //     .replace_range(inter_buf_off.1 - (inline_offset - idx)..inter_buf_off.1, "");
        ctx.curr_cursor_pos.0 = idx;
    }
    update_view_buffer(ctx);
}

pub enum Change {
    DeleteWord(usize, String),
    DeleteSelection(usize, String),
    Delete(usize, char),
    Backspace(usize, char),
    Enter(usize),
    InsertChar(usize, char),
    Paste(usize, String),
    Cut(usize, String),
}

impl undo::Action for Change {
    type Target = Context;
    type Output = ();

    fn apply(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            Change::DeleteWord(idx, s) => {
                target.buffer.buf.replace_range(*idx..*idx + s.len(), "");
            }
            Change::Delete(idx, _) => {
                target.buffer.buf.remove(*idx);
            }
            Change::Backspace(idx, _) => {
                target.buffer.buf.remove(*idx);
            }
            Change::Enter(idx) => {
                target.buffer.buf.insert(*idx, '\n');
            }
            Change::InsertChar(idx, c) => {
                if *c == '\t' {
                    for _ in 0..target.tab_width {
                        target.buffer.buf.insert(*idx, ' ');
                    }
                } else {
                    target.buffer.buf.insert(*idx, *c);
                }
            }
            Change::Paste(idx, s) => {}
            Change::Cut(idx, s) => {}
            Change::DeleteSelection(idx, s) => {
                target
                    .buffer
                    .buf
                    .replace_range(*idx..=(*idx + s.len() - 1), "");
            }
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match self {
            Change::DeleteWord(idx, s) => {
                target.buffer.buf.insert_str(*idx, s);
            }
            Change::Delete(idx, c) => {
                target.buffer.buf.insert(*idx, *c);
            }
            Change::Backspace(idx, c) => {
                target.buffer.buf.insert(*idx, *c);
            }
            Change::Enter(idx) => {
                target.buffer.buf.remove(*idx);
            }
            Change::InsertChar(idx, c) => {
                if *c == '\t' {
                    for _ in 0..target.tab_width {
                        target.buffer.buf.remove(*idx);
                    }
                } else {
                    target.buffer.buf.remove(*idx);
                }
            }
            Change::Paste(idx, s) => {}
            Change::Cut(idx, s) => {}
            Change::DeleteSelection(idx, s) => {
                target.buffer.buf.insert_str(*idx, s);
            }
        }
    }
}

pub async fn update_state(ctx: &mut Context, record: &mut Record<Change>) {
    match get_command() {
        Some(Command::ShiftSelectUp) => {
            if ctx.selection_range.is_none() {
                let init_pos = get_internal_buf_offset(ctx).unwrap().1;
                ctx.selection_range = Some((
                    (init_pos, ctx.curr_cursor_pos),
                    (init_pos, ctx.curr_cursor_pos),
                ));
            }
            move_cursor_up(ctx);
            let curr_pos = get_internal_buf_offset(ctx).unwrap().1;
            ctx.selection_range = Some((
                ctx.selection_range.unwrap().0,
                (curr_pos, ctx.curr_cursor_pos),
            ));
        }
        Some(Command::ShiftSelectLeft) => {
            if ctx.selection_range.is_none() {
                let init_pos = get_internal_buf_offset(ctx).unwrap().1;
                ctx.selection_range = Some((
                    (init_pos, ctx.curr_cursor_pos),
                    (init_pos, ctx.curr_cursor_pos),
                ));
            }
            move_cursor_left(ctx);
            let curr_pos = get_internal_buf_offset(ctx).unwrap().1;
            ctx.selection_range = Some((
                ctx.selection_range.unwrap().0,
                (curr_pos, ctx.curr_cursor_pos),
            ));
        }
        Some(Command::ShiftSelectRight) => {
            if ctx.selection_range.is_none() {
                let init_pos = get_internal_buf_offset(ctx).unwrap().1;
                ctx.selection_range = Some((
                    (init_pos, ctx.curr_cursor_pos),
                    (init_pos, ctx.curr_cursor_pos),
                ));
            }
            move_cursor_right(ctx);
            let curr_pos = get_internal_buf_offset(ctx).unwrap().1;
            ctx.selection_range = Some((
                ctx.selection_range.unwrap().0,
                (curr_pos, ctx.curr_cursor_pos),
            ));
        }
        Some(Command::ShiftSelectDown) => {
            if ctx.selection_range.is_none() {
                let init_pos = get_internal_buf_offset(ctx).unwrap().1;
                ctx.selection_range = Some((
                    (init_pos, ctx.curr_cursor_pos),
                    (init_pos, ctx.curr_cursor_pos),
                ));
            }
            move_cursor_down(ctx);
            let curr_pos = get_internal_buf_offset(ctx).unwrap().1;
            ctx.selection_range = Some((
                ctx.selection_range.unwrap().0,
                (curr_pos, ctx.curr_cursor_pos),
            ));
        }
        Some(Command::ShiftPageUp) => {
            todo!() // TODO
        }
        Some(Command::ShiftPageDown) => {
            todo!() // TODO
        }
        Some(Command::Undo) => {
            record.undo(ctx);
        }
        Some(Command::Redo) => {
            record.redo(ctx);
        }
        Some(Command::Copy) => {
            todo!() // TODO
        }
        Some(Command::Cut) => {
            todo!() // TODO
        }
        Some(Command::Paste) => {
            todo!() // TODO
        }
        Some(Command::OpenDocument) => {
            if let Some(file) = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                .set_directory("/")
                .pick_file()
            {
                ctx.buffer.read_to_buffer(&file);
                ctx.active_buf = file.to_owned();
                update_view_buffer(ctx);
            } else {
                eprintln!("Invalid file selected!");
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
            let line = std::cmp::max(std::cmp::min(ctx.buffer.buf.lines().count(), line), 1);
            ctx.vert_cell_count.0 = line - 1;
            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
            ctx.prompt_input.clear();
            ctx.mode = Modes::Edit;
        }
        Some(Command::MouseLeftClick) => {
            ctx.selection_range = None;
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
                    ctx.curr_cursor_pos = ctx.cells.iter().last().unwrap().pos;
                }
            }
        }
        Some(Command::GoTop) => {
            ctx.selection_range = None;
            ctx.vert_cell_count.0 = 0;
            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
        }
        Some(Command::GoBottom) => {
            ctx.selection_range = None;
            ctx.vert_cell_count.0 = ctx.buffer.buf.lines().count() - 1;
            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
        }
        Some(Command::WordMoveRight) => {
            ctx.selection_range = None;
            move_cursor_right_word(ctx);
        }
        Some(Command::WordMoveLeft) => {
            ctx.selection_range = None;
            move_cursor_left_word(ctx);
        }
        Some(Command::Home) => {
            ctx.selection_range = None;
            ctx.curr_cursor_pos.0 = 0;
        }
        Some(Command::End) => {
            ctx.selection_range = None;
            let view_buffer = from_cells_to_string(&ctx.cells);
            ctx.curr_cursor_pos.0 = view_buffer[ctx.curr_cursor_pos.1]
                .char_indices()
                .last()
                .unwrap()
                .0;
        }
        Some(Command::PageUp) => {
            ctx.selection_range = None;
            ctx.vert_cell_count.0 = std::cmp::max(
                ctx.vert_cell_count
                    .0
                    .saturating_sub(ctx.vert_cell_count.1 as usize),
                0,
            );

            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
        }
        Some(Command::PageDown) => {
            ctx.selection_range = None;
            ctx.vert_cell_count.0 = std::cmp::min(
                ctx.vert_cell_count
                    .0
                    .saturating_add(ctx.vert_cell_count.1 as usize),
                ctx.buffer.buf.lines().count(),
            ) - 1;

            ctx.curr_cursor_pos = (0, 0);
            update_view_buffer(ctx);
        }
        Some(Command::MoveUp) => {
            ctx.selection_range = None;
            move_cursor_up(ctx);
        }
        Some(Command::MoveDown) => {
            ctx.selection_range = None;
            move_cursor_down(ctx);
        }
        Some(Command::MoveLeft) => {
            ctx.selection_range = None;
            move_cursor_left(ctx);
        }
        Some(Command::MoveRight) => {
            ctx.selection_range = None;
            move_cursor_right(ctx);
        }
        Some(Command::DeleteWord) => {
            ctx.selection_range = None;
            ctx.is_file_changed = true;
            delete_word(ctx, record);
        }
        Some(Command::Enter) => {
            if ctx.selection_range.is_some() {
                delete_selection(ctx, record);
            }

            ctx.mode = Modes::Edit;
            ctx.is_file_changed = true;
            let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
            record.apply(ctx, Change::Enter(inter_buf_off.1));
            move_cursor_down(ctx);
            ctx.curr_cursor_pos.0 = 0;
            update_view_buffer(ctx);
        }
        Some(Command::Backspace) => {
            if ctx.selection_range.is_some() {
                ctx.is_file_changed = true;
                delete_selection(ctx, record);
                return;
            }

            let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
            let inline_off = inter_buf_off.get_inline_offset();
            if inter_buf_off.1 == 0 {
                ()
            } else {
                let inline_offset = get_ch_off_to_inline_off(ctx, inter_buf_off.1 - 1);
                ctx.is_file_changed = true;
                record.apply(
                    ctx,
                    Change::Backspace(
                        inter_buf_off.1 - 1,
                        ctx.buffer.buf.chars().nth(inter_buf_off.1 - 1).unwrap(),
                    ),
                );
                // ctx.buffer.buf.remove(inter_buf_off.1 - 1);
                if inline_off != 0 {
                    ctx.curr_cursor_pos.0 -= 1;
                } else {
                    ctx.curr_cursor_pos = (inline_offset, ctx.curr_cursor_pos.1 - 1);
                }
            }
            update_view_buffer(ctx);
        }
        Some(Command::Delete) => {
            if ctx.selection_range.is_some() {
                ctx.is_file_changed = true;
                delete_selection(ctx, record);
                return;
            }

            if get_internal_buf_offset(ctx).unwrap().1
                == ctx.buffer.buf.char_indices().last().unwrap().0
            {
                ()
            } else {
                ctx.is_file_changed = true;
                let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
                record.apply(
                    ctx,
                    Change::Delete(
                        inter_buf_off.1,
                        ctx.buffer.buf.chars().nth(inter_buf_off.1).unwrap(),
                    ),
                );
            }
            update_view_buffer(ctx);
        }
        Some(Command::CharPressed(c)) => {
            if ctx.selection_range.is_some() {
                delete_selection(ctx, record);
            }

            ctx.is_file_changed = true;
            let inter_buf_off = get_internal_buf_offset(ctx).unwrap();
            record.apply(ctx, Change::InsertChar(inter_buf_off.1, c));
            if c == '\t' {
                for _ in 0..ctx.tab_width {
                    ctx.curr_cursor_pos.0 += 1;
                }
            } else {
                ctx.curr_cursor_pos.0 += 1;
            }
            update_view_buffer(ctx);
        }
        None => (),
    }
}
