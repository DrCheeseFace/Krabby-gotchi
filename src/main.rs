use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;

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

    /// number of times to repeat
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
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
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let args = Args::parse();
        let title = Title::from(args.name);
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let mut krabtext: String = "".to_string();
        for _ in 0..self.counter {
            krabtext.push_str("🦀");
        }
        let krabs = Text::from(vec![Line::from(vec!["Value: ".into(), krabtext.into()])]);

        Paragraph::new(krabs)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strokethatthangcuzzo() {
        assert_eq!(true, 1 == 1);
    }
}
