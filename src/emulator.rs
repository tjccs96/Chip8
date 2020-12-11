use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;


pub struct Emulator {
    pub cpu: CPU,
}


impl Emulator {
    pub fn new(path: &Path) -> Self {
        
        //let mut memory: [u8; 4096] = [0; 4096];
        // memory = [0x200..0x200 + rom.len()].copy_from_slice(&rom[..]);
        let mut cpu = CPU::new();
        cpu.load_rom(path);
        // println!("Cpu: {:?}", cpu);
        Self {
            cpu: cpu,
        }
    }



}
