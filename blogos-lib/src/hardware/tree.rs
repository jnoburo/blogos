use super::{BooleanOperation, InputPointer, Instruction};

// Comparisons are structured as a flat vector. Each step contains two values
// and a boolean operation. The comparison fills depth first until it is able to
// reach the circuit input pointers. The circuit then backtracks to the closest
// unfilled space and repeats this process until the entirety of the comparison
// tree is filled. If at any point it bottoms out and does not have a valid next
// step, the comparison will error.
//
// Once the entire tree is built, we can begin pruning it in order to remove
// redundant comparisons and substitute larger comparisons for more efficient
// ones.
pub struct CompTree {
    inputs: usize,
    data: Vec<(BooleanOperation, Option<RawComparison>)>,
    operation: BooleanOperation,
}

impl CompTree {
    pub fn build(
        input_count: usize,
        instructions: Vec<Instruction>,
        operation: BooleanOperation,
    ) -> Self {
        let mut new_instructions: Vec<(BooleanOperation, Option<RawComparison>)> =
            Vec::with_capacity(instructions.len() / 2);

        let mut leaf_instructions = instructions.clone();

        leaf_instructions.sort_by(|a, b| a.a.0.cmp(&a.b.0).cmp(&b.a.0.cmp(&b.b.0)));

        for instruction in leaf_instructions.into_iter() {
            assert!(
                instruction.a.0 <= input_count
                    || instructions.len() >= (instruction.a.0 - input_count)
            );
            assert!(
                instruction.b.0 <= input_count
                    || instructions.len() >= (instruction.b.0 - input_count)
            );
        }

        todo!()
    }
}

// Most basic comparison possible, simply between two raw inputs. From this all
// other comparisons are built.
pub struct RawComparison {
    a: InputPointer,
    b: InputPointer,
    operation: BooleanOperation,
}

pub enum TreeInputs {
    Tree(usize),
    Instruction(Instruction),
}
