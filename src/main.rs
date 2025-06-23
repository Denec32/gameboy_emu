use std::fs;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use gameboy_emu::game_boy::GameBoy;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::{Window, WindowId};

fn decode_ram_size(code: u8) -> String {
    match code { 
        0 => String::from("No RAM"),
        1 => String::from("Unused"),
        2 => String::from("1 bank"),
        3 => String::from("4 banks of 8 KiB each"),
        4 => String::from("16 banks of 8 KiB each"),
        5 => String::from("8 banks of 8 KiB each"),
        _ => panic!("invalid code") 
    }
}

#[derive(Default)]
struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("gameboy emu")
            .with_inner_size(LogicalSize::new(160, 144))
            .with_resizable(false);
        
        self.window = Some(event_loop.create_window(window_attributes).unwrap())
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window {:?} has received the signal to close", window_id);
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                println!("{:?} window redraw", window_id);
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let content = fs::read("hello-world.gb").unwrap();
    
    println!("Cartridge type: {:0x}", content[0x147]);
    println!("ROM size: 32 KiB * {}", 1 << content[0x148]);
    println!("RAM size: {}", decode_ram_size(content[0x149]));

    let mut game_boy = GameBoy::new();
    // game_boy.start(content);
    
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("TODO: panic message");
}