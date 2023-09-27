use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Print(Print),
    Str(Str),
}

#[derive(Debug, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Int {
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    pub value: String,
}
