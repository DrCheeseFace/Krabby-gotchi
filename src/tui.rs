use std::io::{self, stdout, Stdout};
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}














