use rand::Rng;
use std::fs;
use std::path::PathBuf;

use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;


#[derive(Debug)]
pub struct CPU {
    opcode: u16,
    V: [u8; 16],
    I: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    pub memory: [u8; 4096],
    
    pub delay_timer: u8,
    pub sound_timer: u8,

    pub keypad: Keypad,
    pub display: Display,
}

/// Main emulator struct it's called CPU but it is essentially the CPU + Display + Keypad
impl CPU {
    pub fn new() -> Self {
        let mut initial_memory = [0u8; 4096];
        initial_memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
        
        Self {
            opcode: 0,
            memory: initial_memory,
            V: [0; 16],
            I: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }
   

    /// Load rom into memory
    pub fn load_rom(&mut self, rom_path: &PathBuf) -> Result<(), std::io::Error>{
        let game = fs::read(rom_path)?;
        self.memory[(0x200 as usize)..(0x200 as usize + game.len())].copy_from_slice(&game); 
        
        Ok(())
    }

    pub fn decrement_dt(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Fetch opcode 
    fn fetch_opcode(&mut self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16)
    }

    /// Exec a single opcode
    fn exec_opcode(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.V[x];
        let vy = self.V[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // split into nibbles
        // and check which element
        // e.g opcode = 0x1A42, op_1 = 0x1, op_2 = 0xA etc.
        let nibble_1 = (opcode & 0xF000) >> 12;
        let nibble_2 = (opcode & 0x0F00) >> 8;
        let nibble_3 = (opcode & 0x00F0) >> 4;
        let nibble_4 = opcode & 0x000F;

        self.pc += 2;
       

        match (nibble_1, nibble_2, nibble_3, nibble_4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS
                self.display.cls();
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // RET
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x1, _, _, _) => {
                // JP
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                // CALL
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1; 
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                // SE Vx == kk
                if vx == kk {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                // SNE Vx != kk
                if vx != kk {
                    self.pc += 2;
                }
            }
            (0x5, _, _, _) => {
                // SE Vx == Vy
                if vx == vy {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                // LD Vx
                self.V[x] = kk;
            }
            (0x7, _, _, _) => {
                // ADD Vx, byte
                // self.V[x] += kk; was overflowing.
                // Since we dont care about the overflow flag,
                // do wrapping_add.
                self.V[x] = self.V[x].wrapping_add(kk);
            }
            (0x8, _, _, 0x0) => {
                // LD Vx, Vy
                self.V[x] = self.V[y];
            }
            (0x8, _, _, 0x1) => {
                // OR Vx, Vy
                self.V[x] = self.V[x] | self.V[y];
            }
            (0x8, _, _, 0x2) => {
                // AND Vx, Vy
                self.V[x] = self.V[x] & self.V[y];
            }
            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy
                self.V[x] = self.V[x] ^ self.V[y];
            }
            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy
                let (result, overflow) = self.V[x].overflowing_add(self.V[y]);
                self.V[0xF] = if overflow { 1 } else { 0 };
                self.V[x] = result; 
            }
            (0x8, _, _, 0x5) => {
                // SUB Vx, Vy
                let (result, overflow) = self.V[x].overflowing_sub(self.V[y]);
                self.V[0xF] = if overflow { 0 } else { 1 };
                self.V[x] = result;
            }
            (0x8, _, _, 0x6) => {
                // SHR Vx, {, Vy}
                self.V[0xF] = self.V[x] & 0x1; 
                self.V[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy
                let (result, overflow) = self.V[y].overflowing_sub(self.V[x]);
                self.V[0xF] = if overflow { 0 } else { 1 };
                self.V[x] = result;
            }
            (0x8, _, _, 0xE) => {
                //self.V[0xF] = self.V[x] & 0x80;
                self.V[0xF] = self.V[x] >> 7;
                self.V[x] <<= 1;
            }
            (0x9, _, _, _) => {
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                } 
            }
            (0xA, _, _, _) => {
                self.I = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = nnn + self.V[0] as u16;
            }
            (0xC, _, _, _) => {
                // #NOTE Might be slower if i create thread_rng here, could change later
                let rng = rand::thread_rng().gen::<u8>();
                self.V[x] = rng & kk;
            }
            (0xD, _, _, _) => {
               let collision = self.display.draw_sprite(vx as usize, vy as usize,
                                                        &self.memory[self.I as usize .. (self.I + n as u16) as usize]); 
               self.V[0xF] = if collision { 1 } else { 0 }; 
            }
            (0xE, _, 0x9, 0xE) => {
                self.pc += if self.keypad.is_key_down(vx) { 2 } else { 0 };  
            }
            (0xE, _, 0xA, 0x1) => {
                self.pc += if !self.keypad.is_key_down(vx) { 2 } else { 0 }; 
            }
            (0xF, _, 0x0, 0x7) => {
                self.V[x] = self.delay_timer;
            }
            (0xF, _, 0x0, 0xA) => {
                if let Some(key) = self.keypad.get_pressed_key() {
                    self.V[x] = key;
                }
                else {
                    self.pc -= 2;
                }
            }
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.V[x];
            }
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.V[x];
            }
            (0xF, _, 0x1, 0xE) => {
                self.I += self.V[x] as u16;
            }
            (0xF, _, 0x2, 0x9) => {
                self.I = self.V[x] as u16 * 5;
            }
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.I as usize] = self.V[x] / 100;
                self.memory[self.I as usize + 1] = (self.V[x] / 10) % 10;
                self.memory[self.I as usize + 2] = (self.V[x] % 100) % 10;
            }
            (0xF, _, 0x5, 0x5) => {
                self.memory[(self.I as usize)..(self.I + x as u16 + 1) as usize].copy_from_slice(&self.V[0..(x as usize + 1)]);
            }
            (0xF, _, 0x6, 0x5) => {
               self.V[0..(x as usize + 1)].copy_from_slice(&self.memory[(self.I as usize)..(self.I + x as u16 + 1) as usize]); 
            }
            (_, _, _, _) => (),
        }
    }

    pub fn run_cycle(&mut self) {
        let opcode: u16 = self.fetch_opcode(); 
        self.exec_opcode(opcode);
    }
}
