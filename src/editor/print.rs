use super::zipper::{Went, Zipper};
use crate::term::{Branch, Term};

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

fn check<A, B>(x: bool, a: Option<A>, b: Option<B>) -> bool {
    if x {
        b.is_some()
    } else {
        a.is_some()
    }
}

fn print_infix<W: io::Write>(
    w: &mut W,
    context: Context,
    a: Option<&Term>,
    f: Option<&Term>,
    b: Option<&Term>,
    side: bool,
) -> io::Result<()> {
    let x = a.is_none();
    if let Some(a) = a {
        print(w, context, a)?;
    }
    if check(x, a, f) {
        queue!(w, Print(' '))?;
        if !side {
            queue!(w, Print('.'))?;
        }
    }
    if let Some(f) = f {
        print(w, context, f)?;
    }
    if check(x, f, b) {
        if side {
            queue!(w, Print('.'))?;
        }
        queue!(w, Print(' '))?;
    }
    if let Some(b) = b {
        print(w, context, b)?;
    }
    Ok(())
}

fn print_pair<W: io::Write>(
    w: &mut W,
    context: Context,
    open: bool,
    a: Option<&Term>,
    b: Option<&Term>,
    close: bool,
) -> io::Result<()> {
    if open {
        queue!(w, Print('('))?;
    }
    if let Some(a) = a {
        print(w, context, a)?;
    }
    if check(!open, a, b) {
        queue!(w, Print(", "))?;
    }
    if let Some(b) = b {
        print(w, context, b)?;
    }
    if close {
        queue!(w, Print(')'))?;
    }
    Ok(())
}

fn print_apply<W: io::Write>(
    w: &mut W,
    context: Context,
    a: Option<&Term>,
    b: Option<&Term>,
) -> io::Result<()> {
    if let Some(a) = a {
        print(w, context, a)?;
    }
    if check(a.is_none(), a, b) {
        queue!(w, Print(' '))?;
    }
    if let Some(b) = b {
        print(w, context, b)?;
    }
    Ok(())
}

fn print_let<W: io::Write>(
    w: &mut W,
    context: Context,
    n: Option<&String>,
    a: Option<&Term>,
    b: Option<&Term>,
) -> io::Result<()> {
    let x = n.is_none();
    if let Some(n) = n {
        queue!(w, Print(n), Print(": "))?;
    }
    if let Some(a) = a {
        print(w, context.not_simple(), a)?;
    }
    if check(x, a, b) {
        next_line(w, context.indent)?;
    }
    if let Some(b) = b {
        print(w, context, b)?;
    }
    Ok(())
}

fn print_if<W: io::Write>(
    w: &mut W,
    mut context: Context,
    open: bool,
    value: Option<&Term>,
    branches: Option<&Vec<Branch>>,
    branch: Option<(&isize, &String)>,
) -> io::Result<()> {
    if open {
        if !context.simple {
            context = context.next_indent();
            context.simple = true;
            next_line(w, context.indent)?;
        }
        queue!(w, Print("if "))?;
    }
    if let Some(value) = value {
        print(w, context.not_simple(), value)?;
    }
    if let Some(before) = branches {
        for branch in before {
            print_branch(w, context.next_indent(), branch)?;
        }
    }
    if let Some((tag, name)) = branch {
        next_line(w, context.indent)?;
        queue!(w, Print(format!("{} {}: ", tag, name)))?;
    }
    Ok(())
}

fn print_iflet<W: io::Write>(
    w: &mut W,
    mut context: Context,
    default_open: Option<(&isize, &String)>,
    value: Option<&Term>,
    branches: Option<&Vec<Branch>>,
    default_close: Option<&Term>,
    branch: Option<(&isize, &String)>,
) -> io::Result<()> {
    if let Some((tag, name)) = default_open {
        if !context.simple {
            context = context.next_indent();
            context.simple = true;
            next_line(w, context.indent)?;
        }
        queue!(w, Print(format!("{} {}: if ", tag, name)))?;
    }
    if let Some(value) = value {
        print(w, context.not_simple(), value)?;
    }
    if let Some(branches) = branches {
        for branch in branches {
            print_branch(w, context.next_indent(), branch)?;
        }
        next_line(w, context.indent)?;
    }
    if let Some((tag, name)) = branch {
        queue!(w, Print(format!("{} {}: ", tag, name)))?;
    }
    if let Some(block) = default_close {
        print(w, context, block)?;
    }
    Ok(())
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
        Apply(a, b) => print_apply(w, context, Some(a), Some(b))?,
        InfixL(a, f, b) => {
            print_infix(w, context, Some(a), Some(f), Some(b), false)?
        }
        InfixR(a, f, b) => {
            print_infix(w, context, Some(a), Some(f), Some(b), true)?
        }
        Let(n, a, b) => print_let(w, context, Some(n), Some(a), Some(b))?,
        Pair(a, b) => print_pair(w, context, true, Some(a), Some(b), true)?,
        Integer(i) => {
            use crate::term::Integer::*;
            match i {
                I(n) => queue!(w, Print(n))?,
                Add => queue!(w, Print('+'))?,
                Mul => queue!(w, Print('*'))?,
            }
        }
        Branch(_) => todo!(),
        // If(value, branches) => {
        //     print_if(w, context, true, Some(value), Some(branches), None)?
        // }
        If(value, branches) => todo!(),
        // IfLet(value, branches, default) => print_iflet(
        //     w,
        //     context,
        //     Some((&default.tag, &default.name)),
        //     Some(value),
        //     Some(branches),
        //     Some(&*default.block),
        //     None,
        // )?,
        IfLet(value, branches, default) => todo!(),
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
                print_let(w, context, Some(name), None, None)?;
                context.simple = false;
            }
            LetB { name, a } => {
                print_let(w, context, Some(name), Some(a), None)?;
            }
            PairA { .. } => {
                print_pair(w, context, true, None, None, false)?;
            }
            PairB { a } => {
                print_pair(w, context, true, Some(a), None, false)?;
            }
            ApplyB { a } => print_apply(w, context, Some(a), None)?,
            InfixLF { a, .. } => {
                print_infix(w, context, Some(a), None, None, false)?
            }
            InfixLB { a, f } => {
                print_infix(w, context, Some(a), Some(f), None, false)?
            }
            InfixRF { a, .. } => {
                print_infix(w, context, Some(a), None, None, true)?
            }
            InfixRB { a, f } => {
                print_infix(w, context, Some(a), Some(f), None, true)?
            }
            IfValue { .. } => {
                print_if(w, context, true, None, None, None)?;
                context.simple = false;
            }
            IfBranch {
                value,
                before,
                tag,
                name,
                ..
            } => {
                // print_if(
                //     w,
                //     context,
                //     true,
                //     Some(value),
                //     Some(before),
                //     Some((tag, name)),
                // )?;
            }
            IfLetValue { default, .. } => {
                // print_iflet(
                //     w,
                //     context,
                //     Some((&default.tag, &default.name)),
                //     None,
                //     None,
                //     None,
                //     None,
                // )?;
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
                // print_iflet(
                //     w,
                //     context,
                //     Some((&default.tag, &default.name)),
                //     Some(value),
                //     Some(before),
                //     None,
                //     Some((tag, name)),
                // )?;
            }
            IfLetDefault {
                value,
                branches,
                tag,
                name,
            } => {
                // print_iflet(
                //     w,
                //     context,
                //     Some((&tag, &name)),
                //     Some(value),
                //     Some(branches),
                //     None,
                //     None,
                // )?;
                // queue!(w, Print(format!("{} {}: if ", tag, name)))?;
                // print(w, context.not_simple(), value)?;
                // for branch in branches {
                //     print_branch(w, context.next_indent(), branch)?;
                // }

                // next_line(w, context.indent)?;
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
            LetA { b, .. } => print_let(w, context, None, None, Some(b))?,
            PairA { b } => {
                print_pair(w, context, false, None, Some(b), true)?;
            }
            PairB { .. } => {
                print_pair(w, context, false, None, None, true)?;
            }
            ApplyA { b } => print_apply(w, context, None, Some(b))?,
            InfixLA { f, b } => {
                print_infix(w, context, None, Some(f), Some(b), false)?;
            }
            InfixLF { b, .. } => {
                print_infix(w, context, None, None, Some(b), false)?;
            }
            InfixRA { f, b } => {
                print_infix(w, context, None, Some(f), Some(b), true)?;
            }
            InfixRF { b, .. } => {
                print_infix(w, context, None, None, Some(b), true)?;
            }
            IfValue { branches } => {
                print_if(w, context, false, None, Some(branches), None)?;
            }
            IfBranch { after, .. } => {
                // print_if(w, context, false, None, Some(after), None)?;
            }
            IfLetValue { branches, default } => {
                // for branch in branches {
                //     print_branch(w, context.next_indent(), branch)?;
                // }
                // next_line(w, context.indent)?;
                // print(w, context, &*default.block)?;
            }
            IfLetBranch { after, default, .. } => {
                // for branch in after {
                //     print_branch(w, context.next_indent(), branch)?;
                // }
                // next_line(w, context.indent)?;
                // print(w, context, &*default.block)?;
            }
            _ => {}
        }
    }
    Ok(())
}
