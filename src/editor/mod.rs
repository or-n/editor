pub mod command;

use eat::*;

use command::{Command, Migrate};
use std::io;
use std::time::Duration;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Print, ResetColor},
    terminal,
};

pub struct Model {
    pub input: String,
    pub command: Option<Command>,
}

impl Model {
    pub fn run<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        execute!(w, terminal::EnterAlternateScreen, cursor::Show)?;
        terminal::enable_raw_mode()?;
        loop {
            queue!(
                w,
                ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            queue!(
                w,
                cursor::MoveTo(20, 20),
                Print("Command: "),
                Print(&self.input)
            )?;
            w.flush()?;
            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(e) => self.key_event(e),
                    _ => {}
                }
            }
            let command = self.command.clone();
            self.command = None;
            if let Some(command) = command {
                match command {
                    Command::Quit => break,
                    Command::Migrate(m) => {}
                }
            }
        }
        execute!(w, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()
    }

    fn key_event(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Char(c) => self.char_press(c, event.modifiers),
            KeyCode::Backspace => {
                let _ = self.input.pop();
            }
            KeyCode::Enter => {
                let input = self.input.clone();
                self.input.clear();
                match Command::eat(input.as_str(), ()) {
                    Ok(("", command)) => self.command = Some(command),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn char_press(&mut self, c: char, modifiers: KeyModifiers) {
        if modifiers.contains(KeyModifiers::CONTROL) {
            match c {
                'c' => self.command = Some(Command::Quit),
                'h' => self.command = Some(Command::Migrate(Migrate::Up)),
                'j' => self.command = Some(Command::Migrate(Migrate::Right)),
                'k' => self.command = Some(Command::Migrate(Migrate::Left)),
                'l' => self.command = Some(Command::Migrate(Migrate::Down(0))),
                _ => {}
            }
        } else {
            self.input.push(c);
        }
    }
}
