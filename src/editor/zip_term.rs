use crate::term::*;

#[derive(Clone)]
pub struct Zip {
    pub term: Box<T>,
    pub went: Vec<Go>,
}

#[derive(Clone)]
pub enum Go {
    AbstractContinue(String),
    LetValue(Box<T>),
    LetAbstract(Box<T>),
    PairA(Box<T>),
    PairB(Box<T>),
    IfBranch(Vec<T>, Vec<T>),
}

impl Zip {
    pub fn down(&mut self) -> bool {
        match &mut *self.term {
            T::Id(id) => {
                self.term = Box::new(T::Id(id.clone()));
                false
            }
            T::Abstract(id, cont) => {
                self.went.push(Go::AbstractContinue(id.clone()));
                self.term = cont.clone();
                true
            }
            T::Let(value, abstraction) => {
                self.went.push(Go::LetAbstract(value.clone()));
                self.term = abstraction.clone();
                true
            }
            T::I(i64) => {
                self.term = Box::new(T::I(i64.clone()));
                false
            }
            T::Pair(a, b) => {
                self.went.push(Go::PairB(a.clone()));
                self.term = b.clone();
                true
            }
            T::If(ref mut branches) => {
                if let Some(branch) = branches.pop() {
                    self.went.push(Go::IfBranch(branches.clone(), vec![]));
                    self.term = Box::new(branch);
                    true
                } else {
                    self.term = Box::new(T::If(branches.clone()));
                    false
                }
            }
        }
    }

    pub fn up(&mut self) -> bool {
        if let Some(go) = self.went.pop() {
            match go {
                Go::AbstractContinue(id) => {
                    self.term = Box::new(T::Abstract(id, self.term.clone()));
                }
                Go::LetValue(abstraction) => {
                    self.term = Box::new(T::Let(self.term.clone(), abstraction));
                }
                Go::LetAbstract(value) => {
                    self.term = Box::new(T::Let(value, self.term.clone()));
                }
                Go::PairA(b) => {
                    self.term = Box::new(T::Let(self.term.clone(), b));
                }
                Go::PairB(a) => {
                    self.term = Box::new(T::Let(a, self.term.clone()));
                }
                Go::IfBranch(mut before, rev_after) => {
                    before.push(*self.term.clone());
                    before.extend(rev_after.into_iter().rev());
                    self.term = Box::new(T::If(before));
                }
            }
            return true;
        }
        false
    }

    pub fn left(&mut self) -> bool {
        if let Some(go) = self.went.pop() {
            match go {
                Go::AbstractContinue(_) => {
                    self.went.push(go);
                    false
                }
                Go::LetValue(_) => {
                    self.went.push(go);
                    false
                }
                Go::LetAbstract(value) => {
                    self.went.push(Go::LetValue(self.term.clone()));
                    self.term = value;
                    true
                }
                Go::PairA(_) => {
                    self.went.push(go);
                    false
                }
                Go::PairB(a) => {
                    self.went.push(Go::PairA(self.term.clone()));
                    self.term = a;
                    true
                }
                Go::IfBranch(mut before, mut rev_after) => {
                    if let Some(next) = before.pop() {
                        rev_after.push(*self.term.clone());
                        self.term = Box::new(next);
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    pub fn right(&mut self) -> bool {
        if let Some(go) = self.went.pop() {
            match go {
                Go::AbstractContinue(_) => {
                    self.went.push(go);
                    false
                }
                Go::LetValue(abstraction) => {
                    self.went.push(Go::LetAbstract(self.term.clone()));
                    self.term = abstraction;
                    true
                }
                Go::LetAbstract(_) => {
                    self.went.push(go);
                    false
                }
                Go::PairA(b) => {
                    self.went.push(Go::PairB(self.term.clone()));
                    self.term = b;
                    true
                }
                Go::PairB(_) => {
                    self.went.push(go);
                    false
                }
                Go::IfBranch(mut before, mut rev_after) => {
                    if let Some(next) = rev_after.pop() {
                        before.push(*self.term.clone());
                        self.term = Box::new(next);
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
}
