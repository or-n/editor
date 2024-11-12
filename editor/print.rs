use crate::term::{Integer, Term};

#[derive(Clone, Copy)]
pub struct Context;

pub fn print(context: Context, term: &Term) {
    use Term::*;
    match term {
        Parameter(n) => {
            print!("{}", n);
        }
        Apply(a, b) => {
            print(context, a);
            print!(" ");
            print(context, b);
        }
        Let(n, a, b) => {
            print!("{}: ", n);
            print(context, a);
            println!("");
            print(context, b);
        }
        Pair(a, b) => {
            print!("(");
            print(context, a);
            print!(", ");
            print(context, b);
            print!(")");
        }
        Integer(i) => print_integer(context, i),
        _ => {}
    }
}

fn print_integer(_context: Context, i: &Integer) {
    match i {
        Integer::I(n) => {
            print!("{}", n);
        }
        Integer::Add => {
            print!("+");
        }
        Integer::Mul => {
            print!("*");
        }
    }
}
