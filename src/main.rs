mod eval;
mod term;
mod term_new;

use std::collections::HashMap;
use term_new::*;

fn main() {
    let t = id("x".to_string());
    let mut ctx = HashMap::new();
    ctx.insert("x".to_string(), i(69));
    let eval_t = eval::eval(&mut ctx, t);
    println!("{:?}", eval_t);
}
