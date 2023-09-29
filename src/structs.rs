use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    pub value: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Bool(Bool),
    Int(Int),
    Print(Print),
    Str(Str),
    Binary(Binary),
    If(If),
}

#[derive(Debug, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub otherwise: Box<Term>,
    pub then: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub rhs: Box<Term>,
    pub op: Operator,
}

#[derive(Debug, Deserialize)]
pub enum Operator {
    Add,
}

#[derive(Debug, Deserialize)]
pub struct Int {
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    pub value: String,
}
