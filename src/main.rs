mod editor;
mod term;
mod term_from_text;

use eat::*;
use term::*;
use term_from_text::{settings::*, token::Token};

use editor::zipper::Zipper;

use std::collections::HashMap;

fn main() {
    let source = include_str!("examples/iflet");
    let (_, tokens) = Token::eat_many(source, ());
    println!("{:?}", tokens);
    let (rest, term) = BTerm::eat(&tokens[..], Settings::all(true)).unwrap();
    println!("{:?}\n{:?}", rest, term);
    editor::Model {
        input: String::new(),
        command: None,
        zipper: Zipper::new(term.clone()),
    }
    .run(&mut std::io::stdout())
    .unwrap();
    let mut context = HashMap::new();
    println!("{:?}", term.clone().run(&mut context));
}
