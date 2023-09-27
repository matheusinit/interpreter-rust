use crate::structs::Term;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct File {
    pub expression: Term,
}
