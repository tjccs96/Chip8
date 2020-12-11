
#[derive(Debug)]
pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false; 16]
        }

    }

    pub fn is_key_down(&self, index: u8) -> bool {
        self.keys[index as usize]
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for i in 0..self.keys.len() {
            if self.is_key_down(i as u8) {
                return Some(i as u8);
            }
        }
        None
    }
}
