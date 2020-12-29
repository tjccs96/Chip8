use std::path::PathBuf;
use std::rc::Rc;

use chip8::emulator::Emulator;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO need to let the user pick a ROM from a dir
    // instead hard coding it
    let path = PathBuf::from("Roms/TICTAC");
    let emulator = Rc::new(Emulator::new(&path)?);
    emulator.run();
    Ok(())
}
