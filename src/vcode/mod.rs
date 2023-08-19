use self::regalloc::Register;

pub mod regalloc;
pub mod targets;

pub trait Instr {
    fn get_regs(&self) -> Vec<Register>;
}

pub trait ISelector {
    type Instruction: Instr;
}
