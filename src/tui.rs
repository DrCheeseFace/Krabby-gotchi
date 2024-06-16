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
    let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [krab, status] = vertical.areas(frame.size());

    let status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(23),
                Constraint::Percentage(23),
                Constraint::Percentage(23),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(status);

    frame.render_widget(krab_canvas(app), krab);
    frame.render_widget(name_canvas(app), *status_chunks.get(0).unwrap());
    frame.render_widget(hunger_canvas(app), *status_chunks.get(1).unwrap());
    frame.render_widget(happiness_canvas(app), *status_chunks.get(2).unwrap());
    frame.render_widget(health_canvas(app), *status_chunks.get(3).unwrap());
    frame.render_widget(status_canvas(app), *status_chunks.get(4).unwrap());
    frame.render_widget(stage_canvas(app), *status_chunks.get(5).unwrap());

    let help_menu_center_rect: Rect = centered_rect(frame.size(), 30, 30);
    if app.show_help_menu {
        frame.render_widget(
            help_menu_canvas(help_menu_center_rect),
            help_menu_center_rect,
        );
    }
    let save_alert_center_rect: Rect = centered_rect(frame.size(), 10, 10);
    if app.show_save_timer > 0 {
        frame.render_widget(
            save_alert_canvas(app, save_alert_center_rect),
            save_alert_center_rect,
        );
    }
}

pub fn name_canvas(app: &mut App) -> impl Widget + '_ {
    let mut display_name: String = "Name: ".to_string();
    display_name.push_str(app.krab.name());
    Paragraph::new(display_name).block(Block::new())
}
pub fn hunger_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::new().title("Hunger").borders(Borders::TOP))
        .gauge_style(Style::default().fg(match_bar_color(app.krab.hunger())))
        .percent(*app.krab.hunger())
}
pub fn happiness_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::new().title("Happiness"))
        .gauge_style(Style::default().fg(match_bar_color(app.krab.happiness())))
        .percent(*app.krab.happiness())
}
pub fn health_canvas(app: &mut App) -> impl Widget + '_ {
    Gauge::default()
        .block(Block::new().title("Health").borders(Borders::BOTTOM))
        .gauge_style(Style::default().fg(match_bar_color(app.krab.health())))
        .percent(*app.krab.health())
}
pub fn status_canvas(app: &mut App) -> impl Widget + '_ {
    let mut display_mood: String = "Status: ".to_string();
    display_mood.push_str(app.krab.status());
    Paragraph::new(display_mood).block(Block::new())
}
pub fn stage_canvas(app: &mut App) -> impl Widget + '_ {
    let mut display_mood: String = "Stage: ".to_string();
    display_mood.push_str(app.krab.stage());
    Paragraph::new(display_mood).block(Block::new())
}
pub fn krab_canvas(app: &mut App) -> impl Widget + '_ {
    Paragraph::new(app.krab.age().to_string()).block(Block::new())
}

pub fn help_menu_canvas(rect: Rect) -> impl Widget + 'static {
    let text = vec![
        Line::from("Feed -> f"),
        Line::from("Pet -> p"),
        Line::from("Save -> s"),
        Line::from("Toggle Help -> h"),
        Line::from("Quit -> q"),
    ];
    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help Menu")
                .padding(Padding::new(0, 0, rect.height / 4, 0)),
        )
        .alignment(Alignment::Center)
}

pub fn save_alert_canvas(app: &mut App, rect: Rect) -> impl Widget + 'static {
    if app.show_save_timer > 0 {
        app.show_save_timer -= 1;
    }
    Paragraph::new("saved!")
        .block(Block::default().borders(Borders::ALL).padding(Padding::new(
            0,
            0,
            rect.height / 3,
            0,
        )))
        .alignment(Alignment::Center)
}

fn match_bar_color(percent: &u16) -> Color {
    match percent {
        0..=20 => Color::Red,
        21..=50 => Color::Yellow,
        51..=75=> Color::LightGreen,
        76..=100=> Color::Green,
        _ => Color::LightMagenta
    }
}
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
