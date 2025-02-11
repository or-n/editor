use enum_as_inner::EnumAsInner;
use std::collections::HashMap;

pub type Name = String;

pub type BTerm = Box<Term>;

#[derive(Debug, Clone, EnumAsInner)]
pub enum Term {
    Let(Name, BTerm, BTerm),
    Parameter(Name),
    Integer(Integer),
    Pair(BTerm, BTerm),
    Apply(BTerm, BTerm),
    InfixL(BTerm, BTerm, BTerm),
    InfixR(BTerm, BTerm, BTerm),
    Branch(Branch),
    If(BTerm, Vec<BTerm>),
    IfLet(BTerm, Vec<BTerm>, BTerm),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub tag: isize,
    pub name: Name,
    pub block: BTerm,
}

#[derive(Debug, Clone, EnumAsInner)]
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

pub fn branch(tag: isize, name: Name, block: BTerm) -> BTerm {
    Box::new(Term::Branch(Branch { tag, name, block }))
}

pub fn r#if(a: BTerm, b: Vec<BTerm>) -> BTerm {
    Box::new(Term::If(a, b))
}

pub fn iflet(a: BTerm, b: Vec<BTerm>, c: BTerm) -> BTerm {
    Box::new(Term::IfLet(a, b, c))
}

pub fn infixr(a: BTerm, f: BTerm, b: BTerm) -> BTerm {
    Box::new(Term::InfixR(a, f, b))
}

pub fn nil() -> BTerm {
    Box::new(Term::Nil)
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
    pub fn run(self, context: &mut HashMap<Name, BTerm>) -> BTerm {
        use self::Integer::*;
        use Term::*;
        match self {
            Nil => nil(),
            Let(name, a, b) => {
                let a = a.run(context);
                context.insert(name.clone(), a);
                b.run(context)
            }
            Parameter(name) => context.remove(&name).unwrap_or(parameter(name)),
            Integer(i) => integer(i),
            Pair(a, b) => {
                let (a, b) = (*a.run(context), *b.run(context));
                pair(Box::new(a), Box::new(b))
            }
            Apply(a, b) => {
                let (a, b) = (*a.run(context), *b.run(context));
                let result = match a {
                    Integer(Add) => {
                        as_i_pair(&b).map(|(v1, v2)| integer(i(v1 + v2)))
                    }
                    Integer(Mul) => {
                        as_i_pair(&b).map(|(v1, v2)| integer(i(v1 * v2)))
                    }
                    _ => None,
                };
                result.unwrap_or_else(|| apply(Box::new(a), Box::new(b)))
            }
            InfixL(a, f, b) | InfixR(a, f, b) => {
                apply(f, pair(a, b)).run(context)
            }
            Branch(_) => todo!(),
            If(a, b) => {
                let a = *a.run(context);
                let mut try_run_branch = || {
                    // let branches: Option<Vec<super::Branch>> =
                    //     b.into_iter().map(|x| x.as_branch()).collect();
                    // let branches = branches?;
                    let branches: Vec<super::Branch> = todo!();
                    let (tag, value) = a.as_pair()?;
                    let tag_i = tag.as_integer()?.as_i()?;
                    let branch = branches.iter().find(|x| x.tag == *tag_i)?;
                    Some(
                        r#let(
                            branch.name.clone(),
                            value.clone(),
                            branch.block.clone(),
                        )
                        .run(context),
                    )
                };
                try_run_branch().unwrap_or_else(|| r#if(Box::new(a), b))
            }
            IfLet(a, mut branches, default_branch) => {
                branches.push(default_branch);
                If(a, branches).run(context)
            }
        }
    }
}

fn as_i_pair(x: &Term) -> Option<(isize, isize)> {
    let (a, b) = x.as_pair()?;
    Some((*a.as_integer()?.as_i()?, *b.as_integer()?.as_i()?))
}
