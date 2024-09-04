
use alloc::vec::Vec;
use core::mem::size_of;

pub struct VirtualMachine {
    memory: Vec<u8>,
    registers: [u64; 16],
    pc: usize,
}

impl VirtualMachine {
    pub fn new(memory_size: usize) -> Self {
        VirtualMachine {
            memory: vec![0; memory_size],
            registers: [0; 16],
            pc: 0,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory[..program.len()].copy_from_slice(program);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch();
            self.execute(opcode);
        }
    }

    fn fetch(&mut self) -> u32 {
        let instruction = u32::from_le_bytes([
            self.memory[self.pc],
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3],
        ]);
        self.pc += size_of::<u32>();
        instruction
    }

    fn execute(&mut self, opcode: u32) {
        match opcode & 0xF0000000 {
            0x00000000 => self.op_halt(),
            0x10000000 => self.op_load(opcode),
            0x20000000 => self.op_store(opcode),
            0x30000000 => self.op_add(opcode),
            0x40000000 => self.op_sub(opcode),
            0x50000000 => self.op_jump(opcode),
            0x60000000 => self.op_jnz(opcode),
            _ => panic!("Unknown opcode: {:08x}", opcode),
        }
    }

    fn op_halt(&mut self) {
        panic!("Program halted");
    }

    fn op_load(&mut self, opcode: u32) {
        let reg = (opcode & 0x0F000000) >> 24;
        let addr = opcode & 0x00FFFFFF;
        self.registers[reg as usize] = u64::from_le_bytes([
            self.memory[addr as usize],
            self.memory[addr as usize + 1],
            self.memory[addr as usize + 2],
            self.memory[addr as usize + 3],
            self.memory[addr as usize + 4],
            self.memory[addr as usize + 5],
            self.memory[addr as usize + 6],
            self.memory[addr as usize + 7],
        ]);
    }

    fn op_store(&mut self, opcode: u32) {
        let reg = (opcode & 0x0F000000) >> 24;
        let addr = opcode & 0x00FFFFFF;
        let value = self.registers[reg as usize].to_le_bytes();
        self.memory[addr as usize..addr as usize + 8].copy_from_slice(&value);
    }

    fn op_add(&mut self, opcode: u32) {
        let reg1 = (opcode & 0x0F000000) >> 24;
        let reg2 = (opcode & 0x00F00000) >> 20;
        let reg3 = (opcode & 0x000F0000) >> 16;
        self.registers[reg1 as usize] = self.registers[reg2 as usize].wrapping_add(self.registers[reg3 as usize]);
    }

    fn op_sub(&mut self, opcode: u32) {
        let reg1 = (opcode & 0x0F000000) >> 24;
        let reg2 = (opcode & 0x00F00000) >> 20;
        let reg3 = (opcode & 0x000F0000) >> 16;
        self.registers[reg1 as usize] = self.registers[reg2 as usize].wrapping_sub(self.registers[reg3 as usize]);
    }

    fn op_jump(&mut self, opcode: u32) {
        let addr = opcode & 0x0FFFFFFF;
        self.pc = addr as usize;
    }

    fn op_jnz(&mut self, opcode: u32) {
        let reg = (opcode & 0x0F000000) >> 24;
        let addr = opcode & 0x00FFFFFF;
        if self.registers[reg as usize] != 0 {
            self.pc = addr as usize;
        }
    }
}
