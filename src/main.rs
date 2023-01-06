

struct Cpu {
    // RISC-V has 32 registers
    regs: [u64; 32],
    // pc register contains the memory address of next instruction
    pc: u64,
    // memory, a byte-array. There is no memory in real CPU.
    dram: Vec<u8>,
}


// init memory as 128MB
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE - 1;
        Self {regs, pc: 0, dram: code}
    }
}


fn main() {
    println!("Hello, world!");
}
