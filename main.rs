mod digit_text;
mod term;
mod term_text;
mod text;

use term::*;
use text::*;

use term_text::settings::*;

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

fn crab() -> Result<(), ()> {
    let s = "🦀hello";
    let r = s;
    let s = "🦀h".drop(s)?;
    let s = "ell".drop(s)?;
    println!("{r} vs {s}");
    Ok(())
}

fn main() {
    crab();
    let index = Index::new();

    let input = "2137";
    let n = Value::<isize>::try_from(index.value(input)).expect("n");
    println!("{:?}", n.value);

    let result = example().run(&mut HashMap::new());
    println!("{:?}", result);

    let blep_source = include_str!("examples/blep");
    let settings = Settings { apply: true };
    let blep = Value::<BTerm>::try_from(index.value((blep_source, settings))).expect("blep");
    println!("{:?}", blep.value);
}
