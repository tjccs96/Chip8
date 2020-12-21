use std::path::PathBuf;
use std::rc::Rc;

use chip8::emulator::Emulator;


fn main() {
    
    let path = PathBuf::from("Roms/TANK");
    let emulator = Rc::new(Emulator::new(&path));
    emulator.run();
}
