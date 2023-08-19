use std::collections::HashMap;

/// An enum representing all the types of registers
pub enum Register {
    /// Represents a virtual register; there can be infinite and are used like
    /// "seperate" variables.
    ///
    /// Should not be used when lowering to the asm. ONLY IN THE VCODE
    VirtualRegister(usize),

    /// Represents a real register. This corresponds to a register found on the
    /// target you are lowering to and is mapped 1-1.
    RealRegister(usize),

    /// Represents a spilled register. The value stored in these registers on the
    /// target is in RAM
    SpilledRegister(usize),
}

/// A trait representing a register allocator.
pub trait RegisterAllocator {
    fn add_use(&mut self, reg: Register);
    fn add_def(&mut self, reg: Register);
    fn allocate_regs(self) -> HashMap<Register, Register>;
}
