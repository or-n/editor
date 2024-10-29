mod digit_text;
mod term;
mod term_text;
mod text;

use term::*;
use term_text::*;
use text::*;

use std::collections::HashMap;

fn example() -> BTerm {
    let a = integer(i(21));
    let b = integer(i(37));
    let pair = pair(a, b);
    let add = integer(add());
    let apply_add_pair = apply(add, pair);
    let use_c = parameter("c".to_string());
    let0("c".to_string(), apply_add_pair, use_c)
}

fn main() {
    let result = example().run(&mut HashMap::new());
    println!("{:?}", result);

    let settings = Settings {};

    let blep_source = include_str!("examples/blep");
    let blep = BTerm::eat(blep_source, settings);
    println!("{:?}", blep);
}
