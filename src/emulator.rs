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


struct CustomCanvas {
    pub canvas: Canvas<Window>
}

impl CustomCanvas {
    pub fn new(sdl_context: &Sdl) -> Self { 
        let video_subsystem = sdl_context.video().unwrap();
     
        let window = video_subsystem.window("CHIP-8 Emulator", 64 * SCALE, 32 * SCALE)
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
            pixels::Color::RGB(0, 150, 200)
        }
    }

    /// Draws the CPU's display to the canvas
    pub fn draw_canvas(&mut self, cpu: &mut Emulator) {
        for (y, row) in cpu.cpu.display.get_frame_buffer().iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;
            
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
        
        let mut cpu = CPU::new();
        cpu.load_rom(&path);
        Self {
            cpu: cpu,
        }
    }

    pub fn set_keycode(&mut self, code: Keycode, state: bool) {
        match code {
            Keycode::Num1 => { self.cpu.keypad.set(0x1, state); },
            Keycode::Num2 => { self.cpu.keypad.set(0x2, state); },
            Keycode::Num3 => { self.cpu.keypad.set(0x3, state); },
            Keycode::Num4 => { self.cpu.keypad.set(0xC, state); },
            Keycode::Q => { self.cpu.keypad.set(0x4, state); },
            Keycode::W => { self.cpu.keypad.set(0x5, state); },
            Keycode::E => { self.cpu.keypad.set(0x6, state); },
            Keycode::R => { self.cpu.keypad.set(0xD, state); },
            Keycode::A => { self.cpu.keypad.set(0x7, state); },
            Keycode::S => { self.cpu.keypad.set(0x8, state); },
            Keycode::D => { self.cpu.keypad.set(0x9, state); },
            Keycode::F => { self.cpu.keypad.set(0xE, state); },
            Keycode::Z => { self.cpu.keypad.set(0xA, state); },
            Keycode::X => { self.cpu.keypad.set(0x0, state); },
            Keycode::C => { self.cpu.keypad.set(0xB, state); },
            Keycode::V => { self.cpu.keypad.set(0xF, state); },
            _ => {}
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
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
            ::std::thread::sleep(Duration::from_millis(2)); 
            self_mut.cpu.run_cycle();
            self_mut.cpu.decrement_dt();
            canvas.draw_canvas(self_mut);
        }
    }
}
