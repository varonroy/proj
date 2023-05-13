mod app;
mod app_state;
mod styles;

use crate::{config_file::Config, projects::Projects};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use self::{
    app::{draw, handle_input},
    app_state::RunState,
};
use app_state::AppState;

pub fn run(config: Config, projects: Projects) -> anyhow::Result<Option<PathBuf>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app_state = AppState::new(projects);
    let res = main_loop(&mut terminal, config.tick_rate, app_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    terminal.clear()?;

    let exit_dir = res?;

    Ok(exit_dir)
}

fn main_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut app_state: AppState,
) -> anyhow::Result<Option<PathBuf>> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw(f, &mut app_state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            handle_input(&mut app_state)?;
        }
        if last_tick.elapsed() >= tick_rate {
            // on_tick(&mut app_state);
            last_tick = Instant::now();
        }

        match app_state.run_state {
            RunState::Stay => {}
            RunState::Exit => return Ok(None),
            RunState::ExitToDir(path) => return Ok(Some(path)),
        }
    }
}
