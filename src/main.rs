extern crate savefile;
use savefile::prelude::*;
#[macro_use]
extern crate savefile_derive;

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

#[derive(Debug)]
pub struct App {
    exit: bool,
    tick_count: u64,
    name: String,
    krab: Krab,
}

#[derive(Debug, Savefile)]
struct Krab {
    name: String,
    hunger: u8,
    happiness: u8,
    health: u8,
    age: u64,
    weight: u8,
    size: u8,
    mood: String,
    status: String,
}

impl Krab {
    fn new(name: String) -> Self {
        Self {
            name,
            hunger: 0,
            happiness: 0,
            health: 0,
            age: 0,
            weight: 0,
            size: 0,
            mood: String::from("neutral"),
            status: String::from("alive"),
        }
    }
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
            krab: Krab::new("Eugene Krabs".to_string()),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(100);
        self.load_save();
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
        Paragraph::new(self.krab.age.to_string()).block(Block::new().borders(Borders::ALL))
    }
    fn krab_canvas(&self) -> impl Widget + '_ {
        Paragraph::new(self.name.clone()).block(Block::new().borders(Borders::ALL))
    }
    fn buttons_canvas(&self) -> impl Widget + '_ {
        Paragraph::new(self.tick_count.to_string()).block(Block::new().borders(Borders::ALL))
    }

    fn on_tick(&mut self) -> io::Result<()> {
        self.krab.age += 1;
        self.tick_count += 1;
        self.save();
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

    fn save(&self) {
        let save_krab = save_file("krabby-gotchi.save", 0,  &self.krab);
        match save_krab {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to save");
            }
        }
    }

    fn load_save(&mut self) {
        let loaded_krab =load_file::<Krab, &str>("krabby-gotchi.save", 0);
        match loaded_krab {
            Ok(_) => {
                self.krab = loaded_krab.unwrap();
            }
            Err(_) => {
                let newkrab = Krab::new("Eugene Krabs".to_string());
                self.krab = newkrab;
                self.save();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strokethatthangcuzzo() {
        assert_eq!(true, 1 == 1);
    }
}
