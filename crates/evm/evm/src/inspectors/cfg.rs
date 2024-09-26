use revm::{Inspector, EvmContext};
use revm::interpreter::{Interpreter, opcode};
use foundry_evm_core::backend::DatabaseExt;

#[derive(Clone, Debug, Default)]
pub struct OpcodeTracer {
    pub opcode_sequence: Vec<(usize, u8)>, // 存储 opcode 执行序列
    pub cfg_edges: Vec<(usize, usize)>, // 存储 CFG 的边
    pub current_block: Vec<usize>, // 当前正在处理的基本块
}

impl OpcodeTracer {
    pub fn new() -> Self {
        Self {
            opcode_sequence: Vec::new(),
            cfg_edges: Vec::new(),
            current_block: Vec::new(),
        }
    }

    pub fn finalize_block(&mut self, pc: usize) {
        if !self.current_block.is_empty() {
            // 将当前块加入到 CFG 中，处理控制流分支
            // 根据实际需求处理基本块之间的连接逻辑
            self.current_block.clear();
        }
    }

    /// 处理跳转指令，并记录控制流边
    pub fn handle_jump(&mut self, pc: usize, target_pc: usize) {
        self.cfg_edges.push((pc, target_pc));
        self.finalize_block(pc);
    }

    /// 处理普通指令，记录 opcode 序列
    pub fn record_opcode(&mut self, pc: usize, opcode: u8) {
        self.opcode_sequence.push((pc, opcode));
        self.current_block.push(pc); // 将 PC 加入当前基本块
    }
}

impl<DB: DatabaseExt> Inspector<DB> for OpcodeTracer {
    fn step(&mut self, interpreter: &mut Interpreter, _context: &mut EvmContext<DB>) {
        let pc = interpreter.program_counter(); // 获取当前 PC
        let opcode = interpreter.current_opcode(); // 获取当前 opcode
        // self.opcode_sequence.push((pc, opcode)); // 记录 opcode 序列
        // 记录当前的 opcode
        self.record_opcode(pc, opcode);

        // 检查是否是控制流指令
        match opcode {
            opcode::JUMP | opcode::JUMPI => {
                let jump_target = interpreter.stack().peek(0).unwrap().try_into().unwrap(); // 获取跳转目标
                self.handle_jump(pc, jump_target);
            }
            opcode::STOP | opcode::RETURN | opcode::REVERT => {
                self.finalize_block(pc);
            }
            _ => {
                self.current_block.push(pc); // 将当前指令加入基本块
            }
        }
    }

    fn log(&mut self, _interpreter: &mut Interpreter, _context: &mut EvmContext<DB>, _log: &revm::primitives::Log) {
        // 可以处理日志相关信息
    }
}
