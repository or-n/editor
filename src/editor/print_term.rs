use crate::editor::zip_term::*;
use crate::term::*;
use std::io;

pub use crossterm::{
    cursor, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

pub fn inside(range: (usize, usize), x: usize) -> bool {
    x >= range.0 && x <= range.1
}

pub fn abstract_(
    w: &mut impl io::Write,
    range: (usize, usize),
    id: Option<&String>,
    cont: Option<&T>,
) -> io::Result<()> {
    if inside(range, 0) {
        queue!(w, Print("("))?;
    }
    if inside(range, 1) {
        if let Some(id) = id {
            queue!(w, Print(id))?;
        }
    }
    if inside(range, 2) {
        queue!(w, Print(": "))?;
    }
    if inside(range, 3) {
        if let Some(cont) = cont {
            term(w, cont)?;
        }
    }
    if inside(range, 4) {
        queue!(w, Print(")"))?;
    }
    Ok(())
}

pub fn let_(
    w: &mut impl io::Write,
    range: (usize, usize),
    value: Option<&T>,
    abstract_: Option<&T>,
) -> io::Result<()> {
    if inside(range, 0) {
        if let Some(t) = value {
            term(w, t)?;
        }
    }
    if inside(range, 1) {
        queue!(w, Print(": "))?;
    }
    if inside(range, 2) {
        if let Some(t) = abstract_ {
            term(w, t)?;
        }
    }
    Ok(())
}

pub fn zip(w: &mut impl io::Write, x: &Zip) -> io::Result<()> {
    let back = Color::Rgb {
        r: 161,
        g: 63,
        b: 255,
    };
    for go in &x.went {
        go_before(w, go)?;
    }
    queue!(w, SetBackgroundColor(back))?;
    term(w, &x.term)?;
    queue!(w, SetBackgroundColor(Color::Reset))?;
    for go in &x.went {
        go_after(w, go)?;
    }
    Ok(())
}

pub fn term(w: &mut impl io::Write, x: &T) -> io::Result<()> {
    match x {
        T::Id(id) => queue!(w, Print(id))?,
        T::Abstract(id, cont) => abstract_(w, (0, 4), Some(id), Some(cont))?,
        T::Let(value, abstract_) => let_(w, (0, 2), Some(value), Some(abstract_))?,
        T::I(i64) => queue!(w, Print(format!("{}", i64)))?,
        T::Pair(a, b) => {
            queue!(w, Print("("))?;
            term(w, a)?;
            queue!(w, Print(", "))?;
            term(w, b)?;
            queue!(w, Print(")"))?;
        }
        T::If(branches) => {
            for branch in branches {
                term(w, branch)?;
                queue!(w, cursor::MoveToNextLine(1))?;
            }
        }
    }
    Ok(())
}

pub fn go_before(w: &mut impl io::Write, x: &Go) -> io::Result<()> {
    match x {
        Go::AbstractContinue(id) => abstract_(w, (0, 2), Some(id), None)?,
        Go::LetValue(_) => {}
        Go::LetAbstract(value) => let_(w, (0, 1), Some(value), None)?,
        Go::PairA(t) => todo!(),
        Go::PairB(t) => todo!(),
        Go::IfBranch(ts, ts1) => todo!(),
    }
    Ok(())
}

pub fn go_after(w: &mut impl io::Write, x: &Go) -> io::Result<()> {
    match x {
        Go::AbstractContinue(_) => abstract_(w, (4, 4), None, None)?,
        Go::LetValue(abstract_) => let_(w, (1, 2), None, Some(abstract_))?,
        Go::LetAbstract(_) => {}
        Go::PairA(t) => todo!(),
        Go::PairB(t) => todo!(),
        Go::IfBranch(ts, ts1) => todo!(),
    }
    Ok(())
}
