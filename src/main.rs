use std::path::Path;

use futures::executor::block_on;

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu::{Instance, Adapter, Device, Queue, Surface};

use chip8::emulator::Emulator;


fn main() {

    let path = Path::new("Roms/BC_test.ch8");
    let emulator = Emulator::new(path);

    let graphics_instance = Instance::new(wgpu::BackendBit::VULKAN);    
    
    // set winit stuff
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Chip8 EMULATOR").build(&event_loop).unwrap(); 

    // Setup surface
    let surface = unsafe { graphics_instance.create_surface(&window) };
    
    // Setup adapter
    let adapter = block_on(graphics_instance.request_adapter(&wgpu::RequestAdapterOptions {
                                                                power_preference: wgpu::PowerPreference::HighPerformance,
                                                                compatible_surface: Some(&surface)
                                                            })).unwrap();
    // Setup device and queue 
    let (mut device, mut queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                                                                    features: wgpu::Features::default(),
                                                                    limits: wgpu::Limits::default(),
                                                                    shader_validation: false, 
                                                                },None)).unwrap(); 

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;


        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                window.request_redraw();
                // main loop
            },
            _ => ()
        }
    }); 
}
