mod editor;
mod eval;
mod term;
mod term_new;

use editor::zip_term::*;
use term_new::*;

fn main() {
    let x = id("x".to_string());
    let identity = r#abstract("x".to_string(), x);
    let t = r#let(i(69), identity);
    let mut ctx = eval::Ctx::new();
    let mut stdout = std::io::stdout();
    let mut m = editor::Model {
        input: "".to_string(),
        output: "".to_string(),
        mode: editor::Mode::Migrate,
        command: None,
        zip: Zip {
            term: Box::new(t),
            went: vec![],
        },
    };
    let r = m.run(&mut stdout);
    println!("{:?}", r);
    // ctx.insert("x".to_string(), i(69));
    // let eval_t = eval::eval(&mut ctx, t);
    // println!("{:?}", eval_t);
}
