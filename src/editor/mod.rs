pub mod command;
pub mod print_term;
pub mod zip_term;

use eat::*;

use command::{Command, Migrate};
use std::io;
use std::time::Duration;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Print, ResetColor},
    terminal,
};

pub struct Model {
    pub input: String,
    pub output: String,
    pub mode: Mode,
    pub command: Option<Command>,
    pub zip: zip_term::Zip,
}

#[derive(Clone, PartialEq)]
pub enum Mode {
    Migrate,
    Command,
}

impl Model {
    pub fn run<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        execute!(w, terminal::EnterAlternateScreen, cursor::Show)?;
        terminal::enable_raw_mode()?;
        loop {
            let size = terminal::size()?;
            queue!(
                w,
                ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, size.1 - 1),
                Print("Output: "),
                Print(&self.output),
                cursor::MoveTo(0, 1),
            )?;
            queue!(w, cursor::MoveTo(20, 20))?;
            print_term::zip(w, &self.zip)?;
            queue!(w, cursor::MoveTo(0, 0))?;
            if self.mode == Mode::Command {
                queue!(w, Print("Command: "), Print(&self.input),)?;
            }
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
                    Command::Mode(m) => self.mode = m,
                    Command::Migrate(m) => self.migrate(m),
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
            KeyCode::Esc => self.command = Some(Command::Mode(Mode::Migrate)),
            KeyCode::Backspace => {
                let _ = self.input.pop();
            }
            KeyCode::Enter => {
                let input = self.input.clone();
                self.input.clear();
                match Command::eat(input.as_str(), ()) {
                    Ok(("", command)) => self.command = Some(command),
                    _ => self.output = input,
                }
            }
            _ => {}
        }
    }

    fn char_press(&mut self, c: char, modifiers: KeyModifiers) {
        if modifiers.contains(KeyModifiers::CONTROL) {
            match c {
                'c' => self.command = Some(Command::Quit),
                _ => {}
            }
        } else {
            if self.mode == Mode::Migrate {
                match c {
                    ':' => self.command = Some(Command::Mode(Mode::Command)),
                    'h' => self.command = Some(Command::Migrate(Migrate::Left)),
                    'j' => self.command = Some(Command::Migrate(Migrate::Down)),
                    'k' => self.command = Some(Command::Migrate(Migrate::Up)),
                    'l' => self.command = Some(Command::Migrate(Migrate::Right)),
                    _ => {}
                }
            } else {
                self.input.push(c)
            }
        }
    }

    fn migrate(&mut self, m: Migrate) {
        match m {
            Migrate::Up => {
                if !self.zip.up() {
                    self.output = "cannot go up".to_string();
                }
            }
            Migrate::Down => {
                if !self.zip.down() {
                    self.output = "cannot go down".to_string();
                }
            }
            Migrate::Left => {
                if !self.zip.left() {
                    self.output = "cannot go left".to_string();
                }
            }
            Migrate::Right => {
                if !self.zip.right() {
                    self.output = "cannot go right".to_string();
                }
            }
        }
    }
}
