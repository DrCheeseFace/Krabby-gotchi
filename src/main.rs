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
    name: String,
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new(Args::parse().name);
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

impl App {
    fn new(name: String) -> Self {
        Self {
            exit: false,
            tick_count: 0,
            name,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let mut tick_rate = Duration::from_millis(100);
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
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [krab, right] = vertical.areas(frame.size());
        let [status, buttons] = horizontal.areas(right);

        frame.render_widget(self.krab_canvas(), krab);
        frame.render_widget(self.status_canvas(), status);
        frame.render_widget(self.buttons_canvas(), buttons);
    }

    fn status_canvas(&self) -> impl Widget + '_ {
        Paragraph::new("status/stats?").block(Block::new().borders(Borders::ALL))
    }
    fn krab_canvas(&self) -> impl Widget + '_ {
        Paragraph::new(self.name.clone()).block(Block::new().borders(Borders::ALL))
    }
    fn buttons_canvas(&self) -> impl Widget + '_ {
        Paragraph::new(self.tick_count.to_string()).block(Block::new().borders(Borders::ALL))
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
