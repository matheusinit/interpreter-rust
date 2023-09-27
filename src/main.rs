use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Deserialize)]
pub struct File {
    expression: Term,
}
#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Print(Print),
    Str(Str),
}

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(string) => Val::Str(string.value),
        Term::Print(print) => {
            let val = eval(*print.value);
            match val {
                Val::Int(n) => println!("{n}"),
                Val::Bool(b) => println!("{b}"),
                Val::Str(s) => println!("{s}"),
                _ => println!("Erooo"),
            }
            Val::Void
        }
    }
}

fn main() {
    let program = fs::read_to_string("./examples/hello.json").unwrap();
    let program = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;

    eval(term);
}
