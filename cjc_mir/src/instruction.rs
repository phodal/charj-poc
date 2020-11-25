use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    ImportFrom {
        name: String,
    },
    Print {
        // expr: Expression,
    }
}