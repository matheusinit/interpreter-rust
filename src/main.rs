use std::fs;

use crate::eval::eval;
use crate::file::File;

pub mod eval;
pub mod file;
pub mod structs;

fn main() {
    let program = fs::read_to_string("./examples/hello.json").unwrap();
    let program = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;

    eval(term);
}
