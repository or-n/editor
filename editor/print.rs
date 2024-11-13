use crate::term::{Branch, Integer, Term};

use std::io;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Print},
    terminal::{self, ClearType},
    Command,
};

#[derive(Clone, Copy)]
pub struct Context {
    pub indent: usize,
}

pub fn print<W>(w: &mut W, context: Context, term: &Term) -> io::Result<()>
where
    W: io::Write,
{
    use Term::*;
    match term {
        Parameter(n) => {
            queue!(w, Print(n))?;
        }
        Apply(a, b) => {
            print(w, context, a)?;
            queue!(w, Print(' '))?;
            print(w, context, b)?;
        }
        Let(n, a, b) => {
            queue!(w, Print(n), Print(": "))?;
            print(w, context, a)?;
            queue!(w, cursor::MoveToNextLine(1))?;
            print(w, context, b)?;
        }
        Pair(a, b) => {
            queue!(w, Print('('))?;
            print(w, context, a)?;
            queue!(w, Print(", "))?;
            print(w, context, b)?;
            queue!(w, Print(')'))?;
        }
        Integer(i) => print_integer(w, context, i)?,
        IfLet(a, branches, default_branch) => {
            queue!(
                w,
                Print(format!(
                    "{} {}: if ",
                    default_branch.0, default_branch.1.name
                ))
            )?;
            print(w, context, a)?;
            queue!(w, cursor::MoveToNextLine(1))?;
            for b in branches {
                print_branch(
                    w,
                    Context {
                        indent: context.indent + 1,
                    },
                    b,
                )?;
            }
            print(w, context, &*default_branch.1.block)?;
        }
        _ => {}
    }
    Ok(())
}

fn print_branch<W>(
    w: &mut W,
    context: Context,
    (tag, branch): &(isize, Branch),
) -> io::Result<()>
where
    W: io::Write,
{
    print_indent(w, context)?;
    queue!(w, Print(format!("{} {}: ", tag, branch.name)))?;
    print(w, context, &*branch.block)?;
    queue!(w, cursor::MoveToNextLine(1))?;
    Ok(())
}

fn print_indent<W>(w: &mut W, context: Context) -> io::Result<()>
where
    W: io::Write,
{
    for _ in 0..context.indent {
        queue!(w, Print("    "))?;
    }
    Ok(())
}

fn print_integer<W>(w: &mut W, _context: Context, i: &Integer) -> io::Result<()>
where
    W: io::Write,
{
    match i {
        Integer::I(n) => {
            queue!(w, Print(n))?;
        }
        Integer::Add => {
            queue!(w, Print('+'))?;
        }
        Integer::Mul => {
            queue!(w, Print('*'))?;
        }
    }
    Ok(())
}
