use std::path::PathBuf;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use std::time::Duration;

use crate::cpu::CPU;

const SCALE: u32 = 20;


struct CustomKeycodes {
    event: sdl2::EventPump,
}

impl CustomKeycodes {
    pub fn new(sdl_context: &Sdl) -> Self {
        CustomKeycodes {
            event: sdl_context.event_pump().unwrap(),
        }
    }

}


struct CustomCanvas {
    pub canvas: Canvas<Window>
}

impl CustomCanvas {
    pub fn new(sdl_context: &Sdl) -> Self { 
        let video_subsystem = sdl_context.video().unwrap();
     
        let window = video_subsystem.window("rust-sdl2 demo", 64 * SCALE, 32 * SCALE)
            .position_centered()
            .build()
            .unwrap();
     
        let mut canvas = window.into_canvas().build().unwrap();
     
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        
        CustomCanvas {
            canvas: canvas,
        }
    }
    
    pub fn color(&self, value: u8) -> pixels::Color {
        if value == 0 {
            pixels::Color::RGB(0, 0, 0)
        } else {
            pixels::Color::RGB(0, 250, 200)
        }
    }

    /// Draws the CPU's display to the canvas
    pub fn draw_canvas(&mut self, cpu: &mut Emulator) {
        for (y, row) in cpu.cpu.display.get_frame_buffer().iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
                //let x = (i % 64) as u32 * scale;
                //let y = (i / 64) as u32 * scale;
            
                self.canvas.set_draw_color(self.color(col));
                let _ = self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
                
            }
        }
        self.canvas.present();
    }
}



pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new(path: &PathBuf) -> Self {
        
        //let mut memory: [u8; 4096] = [0; 4096];
        // memory = [0x200..0x200 + rom.len()].copy_from_slice(&rom[..]);
        let mut cpu = CPU::new();
        cpu.load_rom(&path);
        //println!("Cpu: {:?}", cpu);
        Self {
            cpu: cpu,
        }
    }

    //fn set_key_state(&mut self, code: VirtualKeyCode, state: bool) {
    //}
    
    pub fn set_keycode(&mut self, code: Keycode, state: bool) {
            let index = match code {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };
            if let Some(i) = index {
                self.cpu.keypad.set(i, state);
                println!("{:?}", self.cpu.keypad);
            }

    }

    pub fn run(mut self: Rc<Self>) {
        let self_mut = Rc::get_mut(&mut self).unwrap();

        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut canvas = CustomCanvas::new(&sdl_context);        
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => self_mut.set_keycode(keycode, true),
                    Event::KeyUp { keycode: Some(keycode), .. } => self_mut.set_keycode(keycode, false),
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
            self_mut.cpu.run_cycle();
            canvas.draw_canvas(self_mut);
        }
    }
}
