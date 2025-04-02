use std::{fs::File, io::Read, vec};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rand::random_range;

use crate::screen::{self, Screen};



//TODO: Refactor OPCODES to functions and a hashmap.
//* INDEV VERSION
pub struct Machine {
    memsiz: usize, // size of memory
    memory:Vec<u8>,
    v: [u8;0x10], // V registries
    i: u16, // Memory addresses
    pc: u16, // Program counter
    stack: [u16; 16],
    sp: u8, // Stack counter
    dt:u8, // Time counter
    st:u8, // Sound counter
}

impl Machine {
    pub fn run(&mut self) {
        self.memsiz = 0x1000;
        let mut i = 0x0;
        let mut pc = self.pc;
        let mut sp = self.sp;
        let mut v = self.v;
        let mut dt = self.dt;
        let mut st = self.st;
        let mut force_close = false;
        let mut pause = false;
        let mut key_pressed = false;
        let mut key_value:u16 = 0;
        let mut font: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];


        self.memory[0x050..0x0A0].copy_from_slice(&font);
        println!("{:?}", self.memory);
        let mut screen = Screen::new(800, 600);
       
        while !force_close {
            let opcode:u16 = ((self.memory[(pc & 0xFFF) as usize] as u16) << 8 | (self.memory[((pc+1) & 0xFFF) as usize]) as u16).into();
            let nnn  = opcode & 0x0FFF;
            let  n = opcode & 0xF;
            let  kk = opcode & 0x00FF;
            let  x = (opcode & 0xF00)>> 8;
            let  y = (opcode & 0x00F0) >> 4;
            let  opcode_index = (opcode & 0xF000) >> 12;
            if pc < (self.memsiz) as u16 && !pause {
                pc +=2
                // print!("{:x}", opcode);

            }else if !pause{
                pc = 0;
            }

            //Reduce timers 60 times per second
            if dt > 0 {
                dt -= 1;
            }
            if st > 0 {
                st -= 1;
            }

            // Handle closure


            if screen.force_close {
                force_close = true;
            }

         
            match opcode_index {
                0x0 => {

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
                            screen.screen_data = vec![0; (screen.width * screen.height) as usize];
                        },
                        _ => ()
                    }
                }
                0x1 => {
                    // The interpreter sets the program counter to nnn.
                    pc = nnn & 0xFFF; 
                }
                0x2 => {
                    //The interpreter increments the stack pointer,
                    //then puts the current PC on the top of the stack.
                    //The PC is then set to nnn.
                    sp +=1;
                    sp = sp & 0x0F;
                    self.stack[sp as usize] = pc;
                    pc = nnn;
                }
                0x3 => {
                    /*
                    Skip next instruction if Vx = kk.
                    The interpreter compares register Vx to kk, and if they are equal,
                    increments the program counter by 2.
                    */
                    if v[x as usize] == kk as u8 {
                        pc += 2;
                    }
                }
                0x4 => {
                    /*
                        Skip next instruction if Vx != kk.
                        The interpreter compares register Vx to kk,
                        and if they are not equal, increments the program counter by 2.                    
                    */
                    if v[x as usize] != kk as u8{
                        pc += 2;
                    }
                }
                0x5 => {
                    /*
                    Skip next instruction if Vx = Vy.
                    The interpreter compares register Vx to register Vy, and if they are equal,
                    increments the program counter by 2.
                    */
                    if v[x as usize] == v[y as usize] {
                        pc += 2;
                    }
                }
                0x6 => {
                    /*
                    Set Vx = kk.
                    The interpreter puts the value kk into register Vx.
                    */
                    v[x as usize] = (kk & 0xFF) as u8;
                }
                0x7 => {
                    /*
                    Set Vx = Vx + kk.
                    Adds the value kk to the value of register Vx,
                    then stores the result in Vx.
                    */
                    v[x as usize] = ((v[x as usize] + (kk as u8)) & 0xFF) as u8;
                }
                0x8 => {
                    match n {
                        0x0 => {
                            /*
                            Set Vx = Vy.
                            Stores the value of register Vy in register Vx.
                            */
                            v[x as usize] = v[y as usize] & 0xFF;
                        }
                        0x1 => {
                            /*
                            Set Vx = Vx OR Vy.
                            Performs a bitwise OR on the values of Vx and Vy,
                            then stores the result in Vx.
                            A bitwise OR compares the corrseponding bits from two values,
                            and if either bit is 1, then the same bit in the result is also 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = (v[x as usize]| v[y as usize]) & 0xFF;
                        }
                        0x2 => {
                            /*
                            Set Vx = Vx AND Vy.
                            Performs a bitwise AND on the values of Vx and Vy, 
                            then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, 
                            and if both bits are 1, then the same bit in the result is also 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = (v[x as usize] & v[y as usize] )& 0xFF;

                        }
                        0x3 => {
                            /*
                            Set Vx = Vx XOR Vy.
                            Performs a bitwise exclusive OR on the values of Vx and Vy,
                            then stores the result in Vx.
                            An exclusive OR compares the corrseponding bits from two values,
                            and if the bits are not both the same,
                            then the corresponding bit in the result is set to 1.
                            Otherwise, it is 0.
                            */
                            v[x as usize] = (v[x as usize] ^ v[y as usize]) & 0xFF;
                        }
                        0x4 => {
                            if (v[x as usize] + v[y as usize]) > 0xFF {
                                v[0xF] = 1;
                            }
                            v[x as usize] =  (v[x as usize] + v[y as usize]) & 0xFF;
                        }
                        0x5 => {
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
                            v[x as usize] = (v[x as usize] - v[y as usize] ) & 0xFF;

                        }
                        0x6 => {
                            /* Set Vx = Vx SHR 1.
                            If the least-significant bit of Vx is 1,
                            then VF is set to 1,
                            otherwise 0. Then Vx is divided by 2.
                            */
                            v[0xF] = v[x as usize] & 0x01; 
                            v[x as usize] >>= 1;
                            v[x as usize] = v[x as usize] & 0xFF;
                        }
                        0x7 => {
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
                            v[x as usize] = (v[y as usize] - v[x as usize]) & 0xFF;
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
                            v[x as usize] = v[x as usize] & 0xFF;
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
                    pc = nnn + v[0] as u16;
                }
                0xC => {
                    /*
                    Set Vx = random byte AND kk.
                    The interpreter generates a random number from 0 to 255, 
                    which is then ANDed with the value kk. 
                    The results are stored in Vx. 
                    See instruction 8xy2 for more information on AND.
                    */
                    v[x as usize] = (random_range(0..255) & (kk as u8)) as u8;
                }
                0xD => {
                    //TODO: Screen DRAW --> Will be done after implementing Screen with SDL
                    /*
                    Dxyn - DRW Vx, Vy, nibble
                    Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                    The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
                    */
                    
                    let x = v[x as usize] & 0x3F; // X (modulo 64)
                    let y = v[y as usize] & 0x1F; // Y (modulo 32)

                    v[0xF] = 0; // Inicializa VF a 0 (sin colisión)
                    for row in 0..n {
                        // if y + row >= 32 { break; } // Stop if out of bounds
                        let sprite = self.memory[(i + row) as usize];
                        for col in 0..8 {
                            // if x + col >= 64 { break; } // Stop if out of bounds
                            let pixel_index = ((y as usize + row as usize) * 64 + (x as usize + col as usize)) as usize;
                            if pixel_index < screen.screen_data.len() {
                                if (sprite & (0x80 >> col)) != 0 {
                                    if screen.screen_data[pixel_index] == 1 {
                                        v[0xF] = 1; // Collision detected
                                    }
                                    screen.screen_data[pixel_index] ^= 1; // XOR operation
                                }
                            }
                        }
                    }
                    println!("I: {:X}, X: {}, Y: {}", i, x, y);
                    println!("Sprite en I: {:X?}, Bytes: {:?}", i, &self.memory[i as usize..(i + n) as usize]);
                    screen.update(); // Update the screen after drawing
                    
                }
                0xE => {
                    match kk {
                        0x9E => {
                            /*
                            Ex9E - SKP Vx
                            Skip next instruction if key with the value of Vx is pressed.
                            Checks the keyboard,
                            and if the key corresponding to the value of Vx is currently in the down position,
                            PC is increased by 2.
                            TODO: Will be done after implementing Screen
                            
                             */
                        }
                        0xA1 => {

                            /*
                            ExA1 - SKNP Vx
                            Skip next instruction if key with the value of Vx is not pressed.
                            Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2. 
                            TODO: Will be done after implementing Screen
                            */
                            pc +=2;

                        }
                        _ => ()
                    }
                }
                0xF => {
                    match kk {
                        0x07 => {
                            /*
                            Set Vx = delay timer value.
                            The value of DT is placed into Vx
                            */
                            v[x as usize] = dt & 0xFF;
                        }
                        0x0A => {
                            /*
                            Fx0A - LD Vx, K
                            Wait for a key press, store the value of the key in Vx.
                            All execution stops until a key is pressed, then the value of that key is stored in Vx.
                            TODO: Will be completed when screen is implemented
                            */
                            
                            if !key_pressed {
                                pause = true
                            }else {
                                v[x as usize] = (key_value & 0x0F) as u8;
                            }
                        }
                        0x15 => {
                            //* Set delay timer to vx value
                            self.dt = v[x as usize] as u8 ;
                        }
                        0x18 => {
                            //* Set sound timer to vx value
                            self.st = v[x as usize] as u8;
                        }
                        0x1E => {
                            if i + v[x as usize] as u16 > 0xFFF {
                                v[0xF] = 1;
                            } else {
                                v[0xF] = 0;
                            }
                            i = i + v[x as usize] as u16;
                        }
                        0x29 => {
                            //TODO: Will be done when screen is implemented.
                            // I = LOCATION OF SPRITE IN VALUE OF VX
                            let char_index = v[x as usize] & 0x0F;
                            i = 0x050 + (char_index as u16 * 5);
                            
                        }
                        0x33 => {
                            /*
                            Fx33 - LD B, Vx
                            Store BCD representation of Vx in memory locations I, I+1, and I+2.
                            The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                            */
                            let value = v[x as usize] as u8;
                            let c = value / 100;
                            let d = (value / 10) % 10;
                            let u = value % 10;
                            self.memory[i as usize] = c;
                            self.memory[(i + (1 as u16)) as usize] = d;
                            self.memory[(i + (2 as u16)) as usize] = u;
                        }

                        0x55 => {
                            /*
                            Store registers V0 through Vx in memory starting at location I.
                            The interpreter copies the values of registers V0 through Vx into memory,
                            starting at the address in I.

                            */
                            for register in 0..=x {
                                self.memory[(i + register)as usize] = v[register as usize] as u8;
                            }
                            i += x + 1;
                        }
                        0x65 => {
                            /*
                            Read registers V0 through Vx from memory starting at location I.
                            The interpreter reads values from memory starting at location I into registers V0 through Vx.
                            */
                            for register in 0..=x {
                                v[register as usize] = self.memory[(i + register) as usize] & 0xFF;
                            }
                            i += x + 1
                        }
                        _ => ()
                    }
                }
                
                _ => {

                }
            };


        }
    }

    pub fn load(&mut self, path:String){
        let mut rom = File::open(path).expect("CANNOT LOAD FILE!");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).expect("Failed to read ROM");
        self.memory[0x200..(0x200 + buffer.len())].copy_from_slice(&buffer);
        let memory = &self.memory;
        let memory_length = memory.len();
    }

    pub fn new() -> Machine{
        Machine{
            memsiz: 4096,
            memory: vec![0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            st: 0,
            dt: 0

        }
    }
}