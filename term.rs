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
}

#[derive(Debug, Clone)]
pub enum Integer {
    I(isize),
    Add,
    Mul,
}

pub fn let0(name: Name, a: BTerm, b: BTerm) -> BTerm {
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
        match self {
            Term::Let(name, a, b) => {
                let a_value = a.run(context);
                context.insert(name.clone(), a_value);
                b.run(context)
            }
            Term::Parameter(name) => context
                .get(name)
                .unwrap_or(&parameter(name.clone()))
                .clone(),
            Term::Integer(i) => i.run(context),
            Term::Pair(a, b) => pair(a.clone(), b.clone()),
            Term::Apply(a, b) => match *a.run(context) {
                Term::Integer(Integer::Add) => match *b.run(context) {
                    Term::Pair(a, b) => match (*a.run(context), *b.run(context)) {
                        (Term::Integer(Integer::I(a)), Term::Integer(Integer::I(b))) => {
                            integer(i(a + b))
                        }
                        (a_value, b_value) => {
                            apply(integer(add()), pair(Box::new(a_value), Box::new(b_value)))
                        }
                    },
                    other => apply(integer(add()), Box::new(other)),
                },
                Term::Integer(Integer::Mul) => match *b.run(context) {
                    Term::Pair(a, b) => match (*a.run(context), *b.run(context)) {
                        (Term::Integer(Integer::I(a)), Term::Integer(Integer::I(b))) => {
                            integer(i(a * b))
                        }
                        (a_value, b_value) => {
                            apply(integer(mul()), pair(Box::new(a_value), Box::new(b_value)))
                        }
                    },
                    other => apply(integer(mul()), Box::new(other)),
                },
                other => apply(Box::new(other), b.clone()),
            },
        }
    }
}

impl Integer {
    pub fn run(&self, _context: &mut HashMap<Name, BTerm>) -> BTerm {
        match self {
            Integer::I(value) => integer(i(*value)),
            _ => integer(self.clone()),
        }
    }
}
