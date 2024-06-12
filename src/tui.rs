use crossterm::{execute, terminal::*};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io::{self, stdout, Stdout};

use crate::App;

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
pub fn render_frame(app: &mut App, frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [krab, bottom] = vertical.areas(frame.size());
    let [status, buttons] = horizontal.areas(bottom);
    let status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(status);
    frame.render_widget(krab_canvas(app), krab);
    frame.render_widget(status_canvas(app), *status_chunks.get(0).unwrap());
    frame.render_widget(status_canvas(app), *status_chunks.get(1).unwrap());
    frame.render_widget(status_canvas(app), *status_chunks.get(2).unwrap());
    frame.render_widget(status_canvas(app), *status_chunks.get(3).unwrap());
    frame.render_widget(buttons_canvas(app), buttons);
}

pub fn status_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.krab.age().to_string()).block(Block::new().borders(Borders::ALL))
}
pub fn krab_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.krab.name().clone()).block(Block::new().borders(Borders::ALL))
}
pub fn buttons_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.tick_count.to_string()).block(Block::new().borders(Borders::ALL))
}
