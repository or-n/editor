use super::zipper::{Went, Zipper};
use crate::term::{Branch, Integer, Term};

use std::io;

pub use crossterm::{
    cursor,
    event::{self, Event},
    queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor},
    terminal::{self, ClearType},
};

#[derive(Clone, Copy)]
pub struct Context {
    pub indent: usize,
}

impl Context {
    pub fn next_indent(&self) -> Self {
        Self {
            indent: self.indent + 1,
        }
    }
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
        IfLet(a, branches, default) => {
            queue!(w, Print(format!("{} {}: if ", default.tag, default.name)))?;
            print(w, context, a)?;
            for b in branches {
                print_branch(w, context.next_indent(), b)?;
            }
            queue!(w, cursor::MoveToNextLine(1))?;
            print(w, context, &*default.block)?;
        }
        _ => {}
    }
    Ok(())
}

fn print_branch<W>(
    w: &mut W,
    context: Context,
    branch: &Branch,
) -> io::Result<()>
where
    W: io::Write,
{
    queue!(w, cursor::MoveToNextLine(1))?;
    print_indent(w, context)?;
    queue!(w, Print(format!("{} {}: ", branch.tag, branch.name)))?;
    print(w, context, &*branch.block)?;
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

pub fn print_zipper<W>(
    w: &mut W,
    context: Context,
    zipper: &Zipper,
) -> io::Result<()>
where
    W: io::Write,
{
    queue!(w, ResetColor)?;
    for went in &zipper.went {
        use Went::*;
        match went {
            LetA { name, .. } => {
                queue!(w, Print(name), Print(": "))?;
            }
            LetB { name, a } => {
                queue!(w, Print(name), Print(": "))?;
                print(w, context, a)?;
                queue!(w, cursor::MoveToNextLine(1))?;
            }
            PairA { .. } => {
                queue!(w, Print('('))?;
            }
            PairB { a } => {
                queue!(w, Print('('))?;
                print(w, context, a)?;
                queue!(w, Print(", "))?;
            }
            ApplyB { a } => {
                print(w, context, a)?;
                queue!(w, Print(' '))?;
            }
            InfixLF { a, .. } => {
                print(w, context, a)?;
                queue!(w, Print(' '))?;
                queue!(w, Print('.'))?;
            }
            InfixLB { a, f } => {
                print(w, context, a)?;
                queue!(w, Print(' '))?;
                queue!(w, Print('.'))?;
                print(w, context, f)?;
                queue!(w, Print(' '))?;
            }
            InfixRF { a, .. } => {
                print(w, context, a)?;
                queue!(w, Print(' '))?;
            }
            InfixRB { a, f } => {
                print(w, context, a)?;
                queue!(w, Print(' '))?;
                print(w, context, f)?;
                queue!(w, Print('.'))?;
                queue!(w, Print(' '))?;
            }
            IfValue { .. } => {
                queue!(w, Print("if "))?;
            }
            IfBranch {
                value,
                before,
                tag,
                name,
                ..
            } => {
                queue!(w, Print("if "))?;
                print(w, context, value)?;
                queue!(w, cursor::MoveToNextLine(1))?;
                for branch in before {
                    print_branch(w, context.next_indent(), branch)?;
                }
                queue!(w, cursor::MoveToNextLine(1))?;
                print_indent(w, context.next_indent())?;
                queue!(w, Print(format!("{} {}: ", tag, name)))?;
            }
            IfLetValue { default, .. } => {
                let iflet = format!("{} {}: if ", default.tag, default.name);
                queue!(w, Print(iflet))?;
            }
            IfLetBranch {
                value,
                before,
                default,
                tag,
                name,
                ..
            } => {
                let iflet = format!("{} {}: if ", default.tag, default.name);
                queue!(w, Print(iflet))?;
                print(w, context, value)?;
                for branch in before {
                    print_branch(w, context.next_indent(), branch)?;
                }
                queue!(w, cursor::MoveToNextLine(1))?;
                print_indent(w, context.next_indent())?;
                queue!(w, Print(format!("{} {}: ", tag, name)))?;
            }
            IfLetDefault {
                value,
                branches,
                tag,
                name,
            } => {
                queue!(w, Print(format!("{} {}: if ", tag, name)))?;
                print(w, context, value)?;
                for branch in branches {
                    print_branch(w, context.next_indent(), branch)?;
                }
                queue!(w, cursor::MoveToNextLine(1))?;
            }
            _ => {}
        }
    }
    queue!(w, SetBackgroundColor(Color::DarkMagenta))?;
    print(w, context, &*zipper.node)?;
    queue!(w, ResetColor)?;
    for went in zipper.went.iter().rev() {
        use Went::*;
        match went {
            LetA { b, .. } => {
                queue!(w, cursor::MoveToNextLine(1))?;
                print(w, context, b)?;
            }
            PairA { b } => {
                queue!(w, Print(", "))?;
                print(w, context, b)?;
                queue!(w, Print(')'))?;
            }
            PairB { .. } => {
                queue!(w, Print(')'))?;
            }
            ApplyA { b } => {
                queue!(w, Print(' '))?;
                print(w, context, b)?;
            }
            InfixLA { f, b } => {
                queue!(w, Print(' '))?;
                queue!(w, Print('.'))?;
                print(w, context, f)?;
                queue!(w, Print(' '))?;
                print(w, context, b)?;
            }
            InfixLF { b, .. } => {
                queue!(w, Print(' '))?;
                print(w, context, b)?;
            }
            InfixRA { f, b } => {
                queue!(w, Print(' '))?;
                print(w, context, f)?;
                queue!(w, Print('.'))?;
                queue!(w, Print(' '))?;
                print(w, context, b)?;
            }
            InfixRF { b, .. } => {
                queue!(w, Print('.'))?;
                queue!(w, Print(' '))?;
                print(w, context, b)?;
            }
            IfValue { branches } => {
                for branch in branches {
                    print_branch(w, context.next_indent(), branch)?;
                }
            }
            IfBranch { after, .. } => {
                for branch in after {
                    print_branch(w, context, branch)?;
                }
            }
            IfLetValue { branches, default } => {
                for branch in branches {
                    print_branch(w, context.next_indent(), branch)?;
                }
                queue!(w, cursor::MoveToNextLine(1))?;
                print(w, context, &*default.block)?;
            }
            IfLetBranch { after, default, .. } => {
                for branch in after {
                    print_branch(w, context.next_indent(), branch)?;
                }
                queue!(w, cursor::MoveToNextLine(1))?;
                print(w, context, &*default.block)?;
            }
            _ => {}
        }
    }
    Ok(())
}
