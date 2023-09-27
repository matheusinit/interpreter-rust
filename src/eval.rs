use crate::structs::Operator;
use crate::structs::Term;
use crate::structs::Val;

pub fn eval(term: Term) -> Val {
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
        Term::Binary(binary) => match binary.op {
            Operator::Add => {
                let right: Val = eval(*binary.rhs);
                let left = eval(*binary.lhs);

                match (left, right) {
                    (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
                    _ => panic!("Cannot add non-ints"),
                }
            }
        },
    }
}
