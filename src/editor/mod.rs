pub mod command;
pub mod print;
pub mod zipper;

use crate::term::*;
use eat::*;

use command::{Command, Migrate, SyntaxItem};
use std::io;
use std::time::Duration;
use zipper::Zipper;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute, queue, terminal,
};

pub struct Model {
    pub input: String,
    pub zipper: Zipper,
    pub command: Option<Command>,
}

impl Model {
    pub fn run<W: io::Write>(&mut self, w: &mut W) -> io::Result<()> {
        if let Some(()) = self.zipper.down(0) {}
        execute!(w, terminal::EnterAlternateScreen, cursor::Show)?;
        terminal::enable_raw_mode()?;
        loop {
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            use print::*;
            print_zipper(
                w,
                Context {
                    indent: 0,
                    simple: true,
                },
                &self.zipper,
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
                    Command::Migrate(m) => migrate(&mut self.zipper, m),
                    Command::Fill(f) => fill(&mut self.zipper, f),
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

fn migrate(zipper: &mut Zipper, migrate: Migrate) {
    use Migrate::*;
    match migrate {
        Up => if let Some(_i) = zipper.up() {},
        Down(i) => if let Some(()) = zipper.down(i) {},
        Left => {
            if let Some(i) = zipper.up() {
                if let None = zipper.down(i - 1) {
                    zipper.down(zipper.kids() - 1).unwrap()
                }
            }
        }
        Right => {
            if let Some(i) = zipper.up() {
                if let None = zipper.down(i + 1) {
                    zipper.down(0).unwrap()
                }
            }
        }
    }
}

fn fill(zipper: &mut Zipper, syntax_item: SyntaxItem) {
    use SyntaxItem::*;
    match syntax_item {
        I(n) => zipper.node = integer(i(n)),
        Add => zipper.node = integer(add()),
        Mul => zipper.node = integer(mul()),
        Pair => zipper.node = pair(nil(), nil()),
        Let(n) => {
            let current = zipper.node.clone();
            zipper.node = r#let(n, nil(), current);
        }
        Parameter(n) => zipper.node = parameter(n),
        If => {
            zipper.node = r#if(
                nil(),
                vec![
                    // Branch {
                    //     tag: 0,
                    //     name: "a".to_string(),
                    //     block: nil(),
                    // },
                    // Branch {
                    //     tag: 1,
                    //     name: "b".to_string(),
                    //     block: nil(),
                    // },
                ],
            )
        }
        IfLet => {
            zipper.node = iflet(
                nil(),
                // vec![Branch {
                //     tag: 1,
                //     name: "b".to_string(),
                //     block: nil(),
                // }],
                vec![],
                branch(0, "a".to_string(), nil()),
            )
        }
        Nil => zipper.node = nil(),
        Apply => {
            let current = zipper.node.clone();
            zipper.node = apply(current, nil());
        }
        _ => {}
    }
}
