use std::{fs::File, io::Read};


pub struct Machine {
    memsiz: usize, // size of memory
    memory:Vec<u8>,
    v: [u16;0xF], // V registries
    i: u16, // Memory addresses
    pc: u16, // Program counter
    stack: [u8; 16],
    sc: u8, // Stack counter
    tc:u8, // Time counter
    soc:u8, // Sound counter
}

impl Machine {
    pub fn run(&mut self) {
        self.memsiz = 0x1000;
        let mut i = 0x0;
        let mut pc = self.pc;
        let force_close = false;
        while !force_close {
            let mut opcode:u16 = ((self.memory[pc as usize] as u16) << 8 | (self.memory[(pc+1) as usize]) as u16).into();
            let mut nnn  = opcode & 0xFFF;
            let mut kk = opcode & 0x00FF;
            let mut x = (opcode & 0xF00)>> 8;
            let mut y = (opcode & 0x00F0) >> 4;
            let mut opcode_index = (opcode & 0xF000) >> 12;
            println!("{opcode_index:x}");
            match opcode_index {
                0 => {
                    println!("0");
                },
                1 => {
                    println!("1");
                },
                _ => {

                }
            };
            if pc < self.memsiz as u16 {
                pc +=1;
            }
        }
    }

    pub fn load(&mut self, path:String){
        let rom = File::open(path).expect("CANNOT LOAD FILE!");
        let data = rom.bytes();
        // println!("{data:?}");
        for byte in data {
            match byte {
                Ok(opcode) => {
                    let _ = &self.memory.push(opcode);
                },
                _ => {panic!("No data!")}
            }
        }
        let memory = &self.memory;
        for byte in memory {
            // print!("{byte:x}")

        }
    }

    pub fn new() -> Machine{
        Machine{
            memsiz: 4096,
            memory: vec![0; 0x200],
            v: [0; 15],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sc: 0,
            soc: 0,
            tc: 0

        }
    }
}