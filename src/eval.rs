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
                    (Val::Int(a), Val::Str(b)) => Val::Str(format!("{a}{b}")),
                    (Val::Str(a), Val::Int(b)) => Val::Str(format!("{a}{b}")),
                    (Val::Str(a), Val::Str(b)) => Val::Str(format!("{a}{b}")),
                    _ => panic!("Cannot add non-ints"),
                }
            }
        },
        Term::If(statement) => {
            let condition = eval(*statement.condition);

            match condition {
                Val::Bool(true) => eval(*statement.then),
                Val::Bool(false) => eval(*statement.otherwise),
                _ => panic!("Something wrong was put in condition for if statement"),
            }
        }
        Term::Bool(bool) => Val::Bool(bool.value),
    }
}
