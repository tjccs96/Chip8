const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    frame_buffer: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    dirty: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            frame_buffer: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            dirty: true,
        }
    }

    pub fn cls(&mut self) {
        self.frame_buffer = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.dirty = true;
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;

        for j in 0..rows {
            // let _row = sprite[j];
            for i in 0..8 {
                let new_y = (y + j) % DISPLAY_HEIGHT;
                let new_x = (x + i) % DISPLAY_WIDTH;
                
                // check if pixel is on
                if (sprite[j] & (0x80 >> i)) != 0x00 {
                    if self.frame_buffer[new_y][new_x] == 0x01 {
                        collision = true;
                    }
                    self.frame_buffer[new_y][new_x] ^= 0x01;
                }
            }
        }
        self.dirty = true;

        collision
    }
}


pub static FONT_SET: [u8; 80] = [
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
