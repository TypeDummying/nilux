
use alloc::vec::Vec;
use core::arch::asm;

pub struct Kernel {
    memory: Vec<u8>,
    registers: [u32; 16],
    pc: usize,
}

impl Kernel {
    pub fn new(memory_size: usize) -> Self {
        Kernel {
            memory: vec![0; memory_size],
            registers: [0; 16],
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.execute(instruction);
        }
    }

    fn fetch(&mut self) -> u32 {
        let instruction = u32::from_le_bytes([
            self.memory[self.pc],
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3],
        ]);
        self.pc += 4;
        instruction
    }

    fn execute(&mut self, instruction: u32) {
        let opcode = instruction >> 28;
        let rd = ((instruction >> 24) & 0xF) as usize;
        let rs = ((instruction >> 20) & 0xF) as usize;
        let rt = ((instruction >> 16) & 0xF) as usize;
        let immediate = instruction & 0xFFFF;

        match opcode {
            0 => self.registers[rd] = self.registers[rs] + self.registers[rt],
            1 => self.registers[rd] = self.registers[rs] - self.registers[rt],
            2 => self.registers[rd] = self.registers[rs] * self.registers[rt],
            3 => {
                if self.registers[rt] != 0 {
                    self.registers[rd] = self.registers[rs] / self.registers[rt];
                }
            },
            4 => self.registers[rd] = self.registers[rs] & self.registers[rt],
            5 => self.registers[rd] = self.registers[rs] | self.registers[rt],
            6 => self.registers[rd] = self.registers[rs] ^ self.registers[rt],
            7 => self.registers[rd] = self.registers[rs] << (self.registers[rt] & 0x1F),
            8 => self.registers[rd] = self.registers[rs] >> (self.registers[rt] & 0x1F),
            9 => self.registers[rd] = immediate as u32,
            10 => self.registers[rd] = self.memory[self.registers[rs] as usize] as u32,
            11 => self.memory[self.registers[rd] as usize] = self.registers[rs] as u8,
            12 => if self.registers[rs] == self.registers[rt] { self.pc = immediate as usize },
            13 => self.pc = (self.registers[rs] as usize + immediate as usize) & 0xFFFFFFFF,
            14 => {
                self.registers[15] = self.pc as u32;
                self.pc = immediate as usize;
            },
            15 => {
                unsafe {
                    asm!("int 0x80");
                }
            },
            _ => {},
        }
    }
}
