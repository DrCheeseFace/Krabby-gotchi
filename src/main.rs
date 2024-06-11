use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io;
use std::time::{Duration, Instant};

mod tui;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "I bestow upon thee a (somewhat) friendly  crustacean"
)]
struct Args {
    /// name the crab
    #[arg(short, long, default_value = "Eugene Krabs")]
    name: String,
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    tick_count: u64,
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

impl App {
    fn new() -> Self {
        Self {
            exit: false,
            tick_count: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut tick_rate = Duration::from_millis(100);
        let mut app = Self::new();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                self.handle_events()?;
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                self.on_tick()?;
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let top_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(top_layout[1]);

        frame.render_widget(
            Paragraph::new("Krab go here").block(Block::new().borders(Borders::ALL)),
            top_layout[0],
        );
        frame.render_widget(
            Paragraph::new("status/stats?").block(Block::new().borders(Borders::ALL)),
            bottom_layout[0],
        );
        frame.render_widget(
            Paragraph::new(self.tick_count.to_string()).block(Block::new().borders(Borders::ALL)),
            bottom_layout[1],
        );
    }

    fn on_tick(&mut self) -> io::Result<()> {
        self.tick_count += 1;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strokethatthangcuzzo() {
        assert_eq!(true, 1 == 1);
    }
}
