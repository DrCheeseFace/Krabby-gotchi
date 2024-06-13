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
                Constraint::Percentage(17),
                Constraint::Percentage(16),
                Constraint::Percentage(17),
                Constraint::Percentage(16),
                Constraint::Percentage(17),
                Constraint::Percentage(17),
            ]
            .as_ref(),
        )
        .split(status);
    frame.render_widget(krab_canvas(app), krab);
    frame.render_widget(buttons_canvas(app), buttons);
    frame.render_widget(name_canvas(app), *status_chunks.get(0).unwrap());
    frame.render_widget(hunger_canvas(app), *status_chunks.get(1).unwrap());
    frame.render_widget(happiness_canvas(app), *status_chunks.get(2).unwrap());
    frame.render_widget(health_canvas(app), *status_chunks.get(3).unwrap());
    frame.render_widget(weight_canvas(app), *status_chunks.get(4).unwrap());
    frame.render_widget(mood_canvas(app), *status_chunks.get(5).unwrap());
}

pub fn name_canvas(app: &mut App) -> impl Widget + '_ {
    let mut display_name: String = "Name: ".to_string();
    display_name.push_str(app.krab.name());
    Paragraph::new(display_name).block(Block::new().borders(Borders::ALL))
}
pub fn hunger_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::bordered().title("Hunger"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .percent(*app.krab.hunger())
}
pub fn happiness_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::bordered().title("Happiness"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .percent(*app.krab.happiness())
}
pub fn health_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::bordered().title("Health"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .percent(*app.krab.health())
}
pub fn weight_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::bordered().title("Weight"))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .percent(*app.krab.weight())
}
pub fn mood_canvas(app: &mut App) -> impl Widget + '_ {
    let mut display_mood: String = "Mood: ".to_string();
    display_mood.push_str(app.krab.mood().to_string().as_str());
    Paragraph::new(display_mood).block(Block::new().borders(Borders::ALL))
}
pub fn krab_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.krab.age().to_string()).block(Block::new().borders(Borders::ALL))
}
pub fn buttons_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.tick_count.to_string()).block(Block::new().borders(Borders::ALL))
}
