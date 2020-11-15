use std::vec::Vec;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct CPU {
    pub opcode: u16,
    // pub memory: [u8; 4096],
    pub memory: Vec<u8>,
    pub V: [u8; 16],
    // index reg
    pub I: u16,
    // program counter
    pub pc: u16,
    pub mem_addr_reg: u16,
    // pub stack: [u16; 16],
    pub stack: Vec<usize>,
    pub sp: u16,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub key: u8,
}

impl CPU {
    pub fn new() -> CPU {
        // might do something else here
        CPU {
            opcode: 0,
            memory: vec![0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            mem_addr_reg: 0,
            //stack: [0; 16],
            stack: vec![],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            key: 0,
        }
    }
    // pub fn fetch_opcode(&mut self) -> () {
    // self.opcode = self.memory[pc] << 8 | memory[pc+1];
    //}

    fn fetch_opcode(&mut self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16)
    }

    fn exec_opcode(&mut self, opcode: u16) -> () {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.V[x];
        let vy = self.V[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                self.stack.push(self.pc as usize);
                self.sp = self.sp + 1;
            }

            _ => ()
        }
    }

    pub fn run_cycle(&mut self) -> () {
        let opcode: u16 = self.fetch_opcode();
        self.exec_opcode(opcode);
    }
}