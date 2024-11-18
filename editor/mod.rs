pub mod print;
pub mod zipper;

use crate::term::Term;
use std::io;
use std::time::Duration;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute, queue, terminal,
};

pub enum Command {
    Fill(SyntaxItem),
    Forget,
    Migrate(Axis<bool>),
}

pub enum Axis<T> {
    Vertical(T),
    Horizontal(T),
}

#[derive(Clone, Copy, Debug)]
pub enum SyntaxItem {
    I,
    Add,
    Mul,
    Pair,
    Let,
    If,
    IfLet,
    Parameter,
    Apply,
    InfixL,
    InfixR,
}

pub struct Model<W: io::Write> {
    pub input: String,
    pub terminate: bool,
    pub w: W,
}

impl<W: io::Write> Model<W> {
    pub fn run(&mut self, t: &Term) -> io::Result<()> {
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
            print(&mut self.w, Context { indent: 0 }, t)?;
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
            if self.terminate {
                break;
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
                self.as_command(input);
            }
            _ => {}
        }
    }

    fn char_press(&mut self, c: char, modifiers: KeyModifiers) {
        if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
            self.terminate = true;
        } else {
            self.input.push(c);
        }
    }

    fn as_command(&mut self, input: String) {
        match input.as_str() {
            "quit" => self.terminate = true,
            _ => {}
        }
    }
}
