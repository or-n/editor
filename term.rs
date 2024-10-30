use std::collections::HashMap;

pub type Name = String;

pub type BTerm = Box<Term>;

#[derive(Debug, Clone)]
pub enum Term {
    Let(Name, BTerm, BTerm),
    Parameter(Name),
    Integer(Integer),
    Pair(BTerm, BTerm),
    Apply(BTerm, BTerm),
    InfixL(BTerm, BTerm, BTerm),
    InfixR(BTerm, BTerm, BTerm),
}

#[derive(Debug, Clone)]
pub enum Integer {
    I(isize),
    Add,
    Mul,
}

pub fn r#let(name: Name, a: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::Let(name, a, b))
}

pub fn parameter(name: Name) -> BTerm {
    Box::new(Term::Parameter(name))
}

pub fn integer(i: Integer) -> BTerm {
    Box::new(Term::Integer(i))
}

pub fn pair(a: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::Pair(a, b))
}

pub fn apply(a: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::Apply(a, b))
}

pub fn infixl(a: BTerm, f: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::InfixL(a, f, b))
}

pub fn infixr(a: BTerm, f: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::InfixR(a, f, b))
}

pub fn i(i: isize) -> Integer {
    Integer::I(i)
}

pub fn add() -> Integer {
    Integer::Add
}

pub fn mul() -> Integer {
    Integer::Mul
}

impl Term {
    pub fn run(&self, context: &mut HashMap<Name, BTerm>) -> BTerm {
        use self::Integer::*;
        use Term::*;
        match self {
            Let(name, a, b) => {
                let a_value = a.run(context);
                context.insert(name.clone(), a_value);
                b.run(context)
            }
            Parameter(name) => context
                .get(name)
                .unwrap_or(&parameter(name.clone()))
                .clone(),
            Integer(i) => integer(i.clone()),
            Pair(a, b) => {
                let a = Box::new(*a.run(context));
                let b = Box::new(*b.run(context));
                pair(a, b)
            }
            Apply(a, b) => match *a.run(context) {
                Integer(Add) => match *b.run(context) {
                    Pair(a, b) => match (*a.run(context), *b.run(context)) {
                        (Integer(I(a)), Integer(I(b))) => integer(i(a + b)),
                        (a_value, b_value) => {
                            let a = Box::new(a_value);
                            let b = Box::new(b_value);
                            apply(integer(add()), pair(a, b))
                        }
                    },
                    other => apply(integer(add()), Box::new(other)),
                },
                Integer(Mul) => match *b.run(context) {
                    Pair(a, b) => match (*a.run(context), *b.run(context)) {
                        (Integer(I(a)), Integer(I(b))) => integer(i(a * b)),
                        (a_value, b_value) => {
                            let a = Box::new(a_value);
                            let b = Box::new(b_value);
                            apply(integer(mul()), pair(a, b))
                        }
                    },
                    other => apply(integer(mul()), Box::new(other)),
                },
                other => apply(Box::new(other), b.clone()),
            },
            InfixL(a, f, b) => apply(f.clone(), pair(a.clone(), b.clone())).run(context),
            InfixR(a, f, b) => apply(f.clone(), pair(a.clone(), b.clone())).run(context),
        }
    }
}
