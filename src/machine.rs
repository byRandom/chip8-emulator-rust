use std::{fs::File, io::Read};




//TODO: Refactor OPCODES to functions and a hashmap.
//* INDEV VERSION
pub struct Machine {
    memsiz: usize, // size of memory
    memory:Vec<u8>,
    v: [u16;0xF], // V registries
    i: u16, // Memory addresses
    pc: u16, // Program counter
    stack: [u16; 16],
    sp: u8, // Stack counter
    tc:u8, // Time counter
    soc:u8, // Sound counter
}

impl Machine {
    pub fn run(&mut self) {
        self.memsiz = 0x1000;
        let mut i = 0x0;
        let mut pc = self.pc;
        let mut sp = self.sp;
        let mut v = self.v;
        let force_close = false;
        println!("{:x}", &self.memory[0x201]);

        while !force_close {
            let opcode:u16 = ((self.memory[(pc & 0xFFF) as usize] as u16) << 8 | (self.memory[((pc +1) & 0xFFF) as usize]) as u16).into();
            let nnn  = opcode & 0xFFF;
            let  n = opcode & 0xF;
            let  kk = opcode & 0x00FF;
            let  x = (opcode & 0xF00)>> 8;
            let  y = (opcode & 0x00F0) >> 4;
            let  opcode_index = (opcode & 0xF000) >> 12;

            // println!("{opcode_index:x}");
            match opcode_index {
                0 => {

                    match nnn {
                        0x0EE => {
                            //Return from a subroutine.
                            //The interpreter sets the program counter to the address at the top of the stack,
                            //then subtracts 1 from the stack pointer.
                            pc = self.stack[sp as usize] as u16;
                            sp = sp - 1;
                        }
                        0x0E0 => {
                            // Clear spreen
                            println!("CLS");
                        },
                        _ => ()
                    }
                }
                1 => {
                    // The interpreter sets the program counter to nnn.
                    pc = nnn & 0xFFF; 
                }
                2 => {
                    //The interpreter increments the stack pointer,
                    //then puts the current PC on the top of the stack.
                    //The PC is then set to nnn.
                    sp +=1;
                    self.stack[sp as usize] = pc;
                    pc = nnn;
                }
                3 => {
                    /*
                    Skip next instruction if Vx = kk.
                    The interpreter compares register Vx to kk, and if they are equal,
                    increments the program counter by 2.
                    */
                    if v[x as usize] == kk {
                        pc += 2;
                    }
                }
                4 => {
                    /*
                        Skip next instruction if Vx != kk.
                        The interpreter compares register Vx to kk,
                        and if they are not equal, increments the program counter by 2.                    
                    */
                    if v[x as usize] != kk {
                        pc += 2;
                    }
                }
                5 => {
                    /*
                    Skip next instruction if Vx = Vy.
                    The interpreter compares register Vx to register Vy, and if they are equal,
                    increments the program counter by 2.
                    */
                    if v[x as usize] == v[y as usize] {
                        pc += 2;
                    }
                }
                6 => {
                    /*
                    Set Vx = kk.
                    The interpreter puts the value kk into register Vx.
                    */
                    v[x as usize] = kk;
                }
                7 => {
                    /*
                    Set Vx = Vx + kk.
                    Adds the value kk to the value of register Vx,
                    then stores the result in Vx.
                    */
                    v[x as usize] = v[x as usize] + kk;
                }
                8 => {
                    match n {
                        0 => {
                            /*
                            Set Vx = Vy.
                            Stores the value of register Vy in register Vx.
                            */
                            v[x as usize] = v[y as usize];
                        }
                        1 => {
                            /*
                            Set Vx = Vx OR Vy.
                            Performs a bitwise OR on the values of Vx and Vy,
                            then stores the result in Vx.
                            A bitwise OR compares the corrseponding bits from two values,
                            and if either bit is 1, then the same bit in the result is also 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = v[x as usize] | v[y as usize];
                        }
                        2 => {
                            /*
                            Set Vx = Vx AND Vy.
                            Performs a bitwise AND on the values of Vx and Vy, 
                            then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, 
                            and if both bits are 1, then the same bit in the result is also 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = v[x as usize] & v[y as usize];

                        }
                        3 => {
                            /*
                            Set Vx = Vx XOR Vy.
                            Performs a bitwise exclusive OR on the values of Vx and Vy,
                            then stores the result in Vx.
                            An exclusive OR compares the corrseponding bits from two values,
                            and if the bits are not both the same,
                            then the corresponding bit in the result is set to 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = v[x as usize] ^ v[y as usize];
                        }
                        4 => {
                            if (v[x as usize] + v[y as usize]) > 0xFF {
                                v[0xF] = 1;
                            }
                            v[x as usize] =  v[x as usize] + v[y as usize];
                        }
                        5 => {

                        }
                        6 => {

                        }
                        7 => {

                        }
                        0xE => {

                        }
                        _ => ()
                    }
                }
                _ => {

                }
            };
            if pc < (self.memsiz) as u16 {
                pc +=1;
                println!("{pc}");
                // print!("{:x}", opcode);

            };
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
        let memory_length = memory.len();
        println!("{memory:x?}");
        if memory_length < 0x1000 {
            let memory_padding = 0x1000 - memory_length;
            let mut padding: Vec<u8> = vec![0; memory_padding];
            let _ = &self.memory.append(&mut padding);
            let new_memory_length = &self.memory.len();
            println!("New memory lenght: {new_memory_length}");
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
            sp: 0,
            soc: 0,
            tc: 0

        }
    }
}