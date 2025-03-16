use std::{fs::File, io::Read};

use rand::random_range;




//TODO: Refactor OPCODES to functions and a hashmap.
//* INDEV VERSION
pub struct Machine {
    memsiz: usize, // size of memory
    memory:Vec<u8>,
    v: [u16;0x10], // V registries
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

        while !force_close {
            let opcode:u16 = ((self.memory[(pc & 0xFFF) as usize] as u16) << 8 | (self.memory[((pc+1) & 0xFFF) as usize]) as u16).into();
            let nnn  = opcode & 0x0FFF;
            let  n = opcode & 0xF;
            let  kk = opcode & 0x00FF;
            let  x = (opcode & 0xF00)>> 8;
            let  y = (opcode & 0x00F0) >> 4;
            let  opcode_index = (opcode & 0xF000) >> 12;
            print!("{}", opcode);
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
                            // Clear screen
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
                            /*
                            Set Vx = Vx - Vy, set VF = NOT borrow.
                            If Vx > Vy, then VF is set to 1,
                            otherwise 0. Then Vy is subtracted from Vx,
                            and the results stored in Vx.
                            */
                            if v[x as usize] > v[y as usize]{
                                v[0xF] = 1;
                            }else{
                                v[0xF] = 0;
                            }
                            v[x as usize] = v[x as usize] - v[y as usize] 

                        }
                        6 => {
                            /* Set Vx = Vx SHR 1.
                            If the least-significant bit of Vx is 1,
                            then VF is set to 1,
                            otherwise 0. Then Vx is divided by 2.
                            */
                            v[0xF] = v[x as usize] & 0x01; 
                            v[x as usize] >>= 1;
                        }
                        7 => {
                            /*
                            Set Vx = Vy - Vx, set VF = NOT borrow.
                            If Vy > Vx, 
                            then VF is set to 1, otherwise 0. 
                            Then Vx is subtracted from Vy, 
                            and the results stored in Vx.
                             */
                            if v[y as usize] > v[x as usize] {
                                v[0xF] = 1;
                            }else{
                                v[0xF] = 0;
                            }
                            v[x as usize] = v[y as usize] - v[x as usize];
                        }
                        0xE => {
                            /* Set Vx = Vx SHL 1.
                            If the most-significant bit of Vx is 1,
                            then VF is set to 1, otherwise to 0. 
                            Then Vx is multiplied by 2.
                            */
                            if (v[x as usize] & 0x80) != 0 {
                                v[0xF] = 1;
                            }else{
                                v[0xF] = 0;
                            }
                            v[x as usize] <<= 1;
                        }
                        _ => ()
                    }
                }
                
                9 => {
                    /*
                    Skip next instruction if Vx != Vy.
                    The values of Vx and Vy are compared,
                    and if they are not equal,
                    the program counter is increased by 2.
                    */
                    if v[x as usize] != v[y as usize]{
                        pc +=2;
                    }
                }

                0xA => {
                    /*
                    Set I = nnn.
                    The value of register I is set to nnn.
                    */
                    i = nnn;
                }
                0xB => {
                    /*
                    Jump to location nnn + V0.
                    The program counter is set to nnn plus the value of V0.
                    */
                    pc = nnn + v[0];
                }
                0xC => {
                    /*
                    Set Vx = random byte AND kk.
                    The interpreter generates a random number from 0 to 255, 
                    which is then ANDed with the value kk. 
                    The results are stored in Vx. 
                    See instruction 8xy2 for more information on AND.
                    */
                    v[x as usize] = random_range(0..255) & kk;
                }
                0xD => {
                    
                }
                0xE => {
                    
                }
                0xF => {
                    
                }
                
                _ => {

                }
            };
            if pc < (self.memsiz) as u16 {
                pc +=1;
                // print!("{:x}", opcode);

            }else{
                pc = 0;
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
        let memory_length = memory.len();
        if memory_length < 0x1000 {
            let memory_padding = 0x1000 - memory_length;
            let mut padding: Vec<u8> = vec![0; memory_padding];
            let _ = &self.memory.append(&mut padding);
        }
    }

    pub fn new() -> Machine{
        Machine{
            memsiz: 4096,
            memory: vec![0; 0x200],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            soc: 0,
            tc: 0

        }
    }
}