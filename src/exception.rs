use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Exception {
    // Riscv Standard Exception
    // InstructionAddrMisaligned(u64),
    // InstructionAccessFault(u64),
    IllegalInstruction(u64),
    // Breakpoint(u64),
    // LoadAccessMisaligned(u64),
    LoadAccessFault(u64),
    // StoreAMOAddrMisaligned(u64),
    StoreAMOAccessFault(u64),
    // EnvironmentCallFromUMode(u64),
    // EnvironmentCallFromSMode(u64),
    // EnvironmentCallFromMMode(u64),
    // InstructionPageFault(u64),
    // LoadPageFault(u64),
    // StoreAMOPageFault(u64),
}

use Exception::*;
impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // InstructionAddrMisaligned(addr) => write!(f, "Instruction address misaligned {:#x}", addr),
            // InstructionAccessFault(addr) => write!(f, "Instruction access fault {:#x}", addr),
            IllegalInstruction(inst) => write!(f, "Illegal instruction {:#x}", inst),
            // Breakpoint(pc) => write!(f, "Breakpoint {:#x}", pc),
            // LoadAccessMisaligned(addr) => write!(f, "Load access {:#x}", addr),
            LoadAccessFault(addr) => write!(f, "Load access fault {:#x}", addr),
            // StoreAMOAddrMisaligned(addr) => write!(f, "Store or AMO address misaliged {:#x}", addr),
            StoreAMOAccessFault(addr) => write!(f, "Store or AMO access fault {:#x}", addr),
            // EnvironmentCallFromUMode(pc) => write!(f, "Environment call from U-mode {:#x}", pc),
            // EnvironmentCallFromSMode(pc) => write!(f, "Environment call from S-mode {:#x}", pc),
            // EnvironmentCallFromMMode(pc) => write!(f, "Environment call from M-mode {:#x}", pc),
            // InstructionPageFault(addr) => write!(f, "Instruction page fault {:#x}", addr),
            // LoadPageFault(addr) => write!(f, "Load page fault {:#x}", addr),
            // StoreAMOPageFault(addr) => write!(f, "Store or AMO page fault {:#x}", addr),
        }
    }
}

