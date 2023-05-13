use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;

use super::{
    app_state::{AppState, RunState},
    styles::styles,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn draw_input<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState, chunk: Rect) {
    let text = app_state.search.as_ref();
    let input = Paragraph::new(text).style(styles().input.style).block(
        Block::default()
            .borders(styles().input.borders)
            .border_type(styles().input.border_type)
            .title(styles().input.title),
    );

    f.render_widget(input, chunk);
    f.set_cursor(chunk.x + text.width() as u16 + 1, chunk.y + 1);
}

pub fn draw_list<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState, chunk: Rect) {
    let items = app_state
        .search_results()
        .into_iter()
        .map(|search_result| {
            ListItem::new(vec![
                Spans::from(
                    search_result
                        .name
                        .chars()
                        .enumerate()
                        .map(|(i, ch)| {
                            if search_result.fuzzy_match.contains(&i) {
                                Span::styled(ch.to_string(), styles().list.matched_letter)
                            } else {
                                Span::styled(ch.to_string(), styles().list.normal_letter)
                            }
                        })
                        .collect_vec(),
                ),
                Spans::from(Span::styled(search_result.dir, styles().list.faded_letter)),
            ])
            .style(Style::default())
        })
        .collect_vec();

    let block = Block::default()
        .borders(styles().list.borders)
        .border_type(styles().list.border_type);

    let items = List::new(items)
        .block(block)
        .highlight_symbol(styles().list.highligh_symbol);

    f.render_stateful_widget(items, chunk, app_state.list_state());
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(styles().v_margin)
        .horizontal_margin(styles().h_margin)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(f.size());

    draw_input(f, app_state, chunks[0]);
    draw_list(f, app_state, chunks[1]);
}

pub fn handle_input(app_state: &mut AppState) -> anyhow::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Esc => app_state.run_state = RunState::Exit,
            KeyCode::Char(ch) => app_state.character_typed(ch),
            KeyCode::Backspace => app_state.backspace_typed(),
            KeyCode::Down => app_state.down_typed(),
            KeyCode::Up => app_state.up_typed(),
            KeyCode::Enter => {
                if let Some(project) = app_state.selected() {
                    app_state.run_state = RunState::ExitToDir(project.dir.to_path_buf());
                }
            }
            _ => {}
        }
    }
    Ok(())
}

// pub fn on_tick(app_state: &mut AppState) {
//     app_state.update_search();
// }
