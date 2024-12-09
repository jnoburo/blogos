use std::collections::hash_map::IntoIter;

use ndarray::Data;

pub type InputPointer = (usize, bool);

#[derive(Debug, Clone, Copy)]
pub enum BooleanOperation {
    And,
    Or,
}

impl BooleanOperation {
    pub const fn negate(&self) -> Self {
        match self {
            Self::And => Self::Or,
            Self::Or => Self::And,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub a: InputPointer,
    pub b: InputPointer,
    pub operation: BooleanOperation,
}

impl Instruction {
    pub const fn new(a: InputPointer, b: InputPointer, operation: BooleanOperation) -> Instruction {
        Instruction { a, b, operation }
    }

    pub const fn negate(&self) -> Self {
        Instruction {
            a: (self.a.0, !self.a.1),
            b: (self.b.0, !self.b.1),
            operation: self.operation.negate(),
        }
    }

    pub fn eval(&self, inputs: Vec<bool>) -> bool {
        let con1: bool = (self.a.1 == inputs[self.a.0]);
        let con2: bool = (self.b.1 == inputs[self.b.0]);
        match self.operation {
            BooleanOperation::And => return con1 && con2,
            BooleanOperation::Or => return con1 || con2,
        }
    }
}

