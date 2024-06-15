extern crate savefile;
use savefile::prelude::*;
#[macro_use]
extern crate savefile_derive;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;
use std::time::{Duration, Instant};
mod krab;
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
    krab: krab::Krab,
    show_help_menu: bool,
    show_save_timer: u16,
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new(Args::parse().name.clone());
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

impl App {
    fn new(name: String) -> Self {
        Self {
            exit: false,
            tick_count: 0,
            krab: krab::Krab::new(name),
            show_help_menu: false,
            show_save_timer: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(100);
        self.load_save();
        while !self.exit {
            terminal.draw(|frame| tui::render_frame(self, frame))?;
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

    fn on_tick(&mut self) -> io::Result<()> {
        self.tick_count += 1;
        if self.tick_count % 600 == 0 {
            self.krab.grow_older();
            self.save(true);
        }
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
            KeyCode::Char('f') => self.krab.feed(),
            KeyCode::Char('p') => self.krab.pet(),
            KeyCode::Char('s') => self.save(true),
            KeyCode::Char('h') => self.toggle_help(),
            _ => {}
        }
    }

    fn toggle_help(&mut self) {
        self.show_help_menu = !self.show_help_menu;
    }

    fn exit(&mut self) {
        self.exit = true;
        self.save(false);
    }

    fn save(&mut self, show_save: bool) {
        let save_krab = save_file("krabby-gotchi.save", 0, &self.krab);
        match save_krab {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to save");
            }
        }
        if show_save {
            self.show_save_timer = 10;
        }
    }

    fn load_save(&mut self) {
        let loaded_krab = load_file::<krab::Krab, &str>("krabby-gotchi.save", 0);
        match loaded_krab {
            Ok(_) => {
                self.krab = loaded_krab.unwrap();
            }
            Err(_) => {
                let newkrab = krab::Krab::new(Args::parse().name.clone());
                self.krab = newkrab;
                self.save(false);
            }
        }
    }
}
