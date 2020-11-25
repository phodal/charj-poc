use crate::basic_block::BasicBlock;
use cjc_lexer::Location;
use crate::instruction::MIRKind;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub name: String,
    pub block: BasicBlock,
}

#[allow(dead_code)]
impl ControlFlowGraph {
    pub fn placeholder(name: String, block: BasicBlock) -> Self {
        ControlFlowGraph { name, block }
    }

    fn emit(&mut self, instruction: MIRKind, _location: Location) {
        self.block.instructions.push(instruction);
    }
}
