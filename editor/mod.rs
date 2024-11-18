pub mod command;
pub mod print;
pub mod zipper;

use crate::util::text::Eat;

use command::{Command, Migrate};
use std::io;
use std::time::Duration;
use zipper::Zipper;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute, queue, terminal,
};

pub struct Model<W: io::Write> {
    pub input: String,
    pub zipper: Zipper,
    pub command: Option<Command>,
    pub w: W,
}

impl<W: io::Write> Model<W> {
    pub fn run(&mut self) -> io::Result<()> {
        execute!(self.w, terminal::EnterAlternateScreen, cursor::Show)?;
        terminal::enable_raw_mode()?;
        loop {
            queue!(
                self.w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            use print::*;
            print_zipper(&mut self.w, Context { indent: 0 }, &self.zipper)?;
            queue!(
                self.w,
                cursor::MoveTo(20, 20),
                Print("Command: "),
                Print(&self.input)
            )?;
            self.w.flush()?;
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
                    Command::Migrate(migrate) => match migrate {
                        Migrate::Up => if let Some(_i) = self.zipper.up() {},
                        Migrate::Down(i) => {
                            if let Some(()) = self.zipper.down(i) {}
                        }
                        Migrate::Left => {
                            if let Some(i) = self.zipper.up() {
                                if let None = self.zipper.down(i - 1) {
                                    if let Some(()) =
                                        self.zipper.down(self.zipper.kids() - 1)
                                    {
                                    }
                                }
                            }
                        }
                        Migrate::Right => {
                            if let Some(i) = self.zipper.up() {
                                if let None = self.zipper.down(i + 1) {
                                    if let Some(()) = self.zipper.down(0) {}
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        execute!(self.w, terminal::LeaveAlternateScreen)?;
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
                match Command::eat(&input, ()) {
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
