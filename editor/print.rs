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
    pub simple: bool,
}

impl Context {
    pub fn next_indent(self) -> Self {
        Self {
            indent: self.indent + 1,
            ..self
        }
    }

    pub fn not_simple(self) -> Self {
        Self {
            simple: false,
            ..self
        }
    }
}

pub fn print<W>(w: &mut W, mut context: Context, term: &Term) -> io::Result<()>
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
        InfixL(a, f, b) => {
            print(w, context, a)?;
            queue!(w, Print(' '))?;
            queue!(w, Print('.'))?;
            print(w, context, f)?;
            queue!(w, Print(' '))?;
            print(w, context, b)?;
        }
        InfixR(a, f, b) => {
            print(w, context, a)?;
            queue!(w, Print(' '))?;
            print(w, context, f)?;
            queue!(w, Print('.'))?;
            queue!(w, Print(' '))?;
            print(w, context, b)?;
        }
        Let(n, a, b) => {
            queue!(w, Print(n), Print(": "))?;
            print(w, context.not_simple(), a)?;
            next_line(w, context.indent)?;
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
        If(value, branches) => {
            if !context.simple {
                context = context.next_indent();
                context.simple = true;
                next_line(w, context.indent)?;
            }
            queue!(w, Print("if "))?;
            print(w, context.not_simple(), value)?;
            for branch in branches {
                print_branch(w, context.next_indent(), branch)?;
            }
        }
        IfLet(a, branches, default) => {
            if !context.simple {
                context = context.next_indent();
                context.simple = true;
                next_line(w, context.indent)?;
            }
            queue!(w, Print(format!("{} {}: if ", default.tag, default.name)))?;
            print(w, context.not_simple(), a)?;
            for branch in branches {
                print_branch(w, context.next_indent(), branch)?;
            }
            next_line(w, context.indent)?;
            print(w, context, &*default.block)?;
        }
        Nil => queue!(w, Print("()"))?,
    }
    Ok(())
}

fn next_line<W: io::Write>(w: &mut W, indent: usize) -> io::Result<()> {
    queue!(
        w,
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(indent as u16 * 4)
    )
}

fn print_branch<W>(
    w: &mut W,
    context: Context,
    branch: &Branch,
) -> io::Result<()>
where
    W: io::Write,
{
    next_line(w, context.indent)?;
    queue!(w, Print(format!("{} {}: ", branch.tag, branch.name)))?;
    print(w, context, &*branch.block)?;
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
    mut context: Context,
    zipper: &Zipper,
) -> io::Result<()>
where
    W: io::Write,
{
    queue!(w, ResetColor)?;
    let context_backup = context;
    for went in &zipper.went {
        use Went::*;
        match went {
            LetA { name, .. } => {
                queue!(w, Print(name), Print(": "))?;
                context.simple = false;
            }
            LetB { name, a } => {
                queue!(w, Print(name), Print(": "))?;
                print(w, context.not_simple(), a)?;
                next_line(w, context.indent)?;
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
                context.simple = false;
            }
            IfBranch {
                value,
                before,
                tag,
                name,
                ..
            } => {
                queue!(w, Print("if "))?;
                print(w, context.not_simple(), value)?;
                for branch in before {
                    print_branch(w, context.next_indent(), branch)?;
                }
                next_line(w, context.next_indent().indent)?;
                queue!(w, Print(format!("{} {}: ", tag, name)))?;
            }
            IfLetValue { default, .. } => {
                let iflet = format!("{} {}: if ", default.tag, default.name);
                queue!(w, Print(iflet))?;
                context.simple = false;
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
                print(w, context.not_simple(), value)?;
                for branch in before {
                    print_branch(w, context.next_indent(), branch)?;
                }
                next_line(w, context.next_indent().indent)?;
                queue!(w, Print(format!("{} {}: ", tag, name)))?;
            }
            IfLetDefault {
                value,
                branches,
                tag,
                name,
            } => {
                queue!(w, Print(format!("{} {}: if ", tag, name)))?;
                print(w, context.not_simple(), value)?;
                for branch in branches {
                    print_branch(w, context.next_indent(), branch)?;
                }

                next_line(w, context.indent)?;
            }
            _ => {}
        }
    }
    queue!(w, SetBackgroundColor(Color::Cyan))?;
    print(w, context, &*zipper.node)?;
    queue!(w, ResetColor)?;
    context = context_backup;
    for went in zipper.went.iter().rev() {
        use Went::*;
        match went {
            LetA { b, .. } => {
                next_line(w, context.indent)?;
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
                    print_branch(w, context.next_indent(), branch)?;
                }
            }
            IfLetValue { branches, default } => {
                for branch in branches {
                    print_branch(w, context.next_indent(), branch)?;
                }
                next_line(w, context.indent)?;
                print(w, context, &*default.block)?;
            }
            IfLetBranch { after, default, .. } => {
                for branch in after {
                    print_branch(w, context.next_indent(), branch)?;
                }
                next_line(w, context.indent)?;
                print(w, context, &*default.block)?;
            }
            _ => {}
        }
    }
    Ok(())
}
