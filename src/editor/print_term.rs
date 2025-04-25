use crate::term::*;
use std::io;

pub use crossterm::{cursor, queue, style::Print};

pub fn term(w: &mut impl io::Write, x: &T) -> io::Result<()> {
    match x {
        T::Id(id) => queue!(w, Print(id))?,
        T::Abstract(id, cont) => {
            queue!(w, Print("("))?;
            queue!(w, Print(id), Print(": "))?;
            term(w, cont)?;
            queue!(w, Print(")"))?;
        }
        T::Let(value, cont) => {
            term(w, value)?;
            queue!(w, Print(" "))?;
            term(w, cont)?;
        }
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
