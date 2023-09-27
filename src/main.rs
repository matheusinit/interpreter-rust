use std::{env, fs};

use crate::eval::eval;
use crate::file::File;

pub mod eval;
pub mod file;
pub mod structs;

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let program = fs::read_to_string(input_path).unwrap();
    let program = serde_json::from_str::<File>(&program).unwrap();

    let term = program.expression;

    eval(term);
}
