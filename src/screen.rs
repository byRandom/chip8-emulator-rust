extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    pixel_size: u32,
    pub screen_data: Vec<u8>,
    pub screen_buffer: Vec<u8>,
    pub event_pump: sdl2::EventPump,
    pub force_close: bool,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    started: bool,
}


impl Screen {
    pub fn update(&mut self) {
        if !self.started{
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));


            self.canvas.clear();
            self.canvas.fill_rect(sdl2::rect::Rect::new(0, 0, self.width, self.height)).unwrap();            
            self.canvas.present();
            self.started = true;
            println!("Screen started");
        }
            
        

        
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.force_close = true;
                },
                _ => {}
            }
        }



        // The rest of the game loop goes here...


        //We draw the screen data
        // If pixel is not different from screen buffer, we draw it

        for pixel in 0..self.screen_data.len() {
            if self.screen_buffer[pixel] != self.screen_data[pixel] {
                if self.screen_data[pixel] == 1 {
                    let x = (pixel % 64) as i32;
                    let y = (pixel / 64) as i32;
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    self.canvas.fill_rect(sdl2::rect::Rect::new(x * self.pixel_size as i32, y * self.pixel_size as i32, self.pixel_size, self.pixel_size)).unwrap();
                }else if self.screen_data[pixel] == 0 {
                    let x = (pixel % 64) as i32;
                    let y = (pixel / 64) as i32;
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                    self.canvas.fill_rect(sdl2::rect::Rect::new(x * self.pixel_size as i32, y * self.pixel_size as i32, self.pixel_size, self.pixel_size)).unwrap();
                }
            }
        }
            
        self.screen_buffer = self.screen_data.clone();
        
        
        
        self.canvas.present();
        
         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn close (&mut self) {
        self.force_close = true;
    }
    
    pub fn new(width:u32, heigth:u32) -> Screen {
        fn create_canvas(width: u32, heigth: u32) -> sdl2::render::Canvas<sdl2::video::Window> {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            let window = video_subsystem.window("chip-8 Emulator", width, heigth)
                .position_centered()
                .build()
                .unwrap();
        
            window.into_canvas().build().unwrap()
        }
        
        fn create_event_pump() -> sdl2::EventPump {
            let sdl_context = sdl2::init().unwrap();
            sdl_context.event_pump().unwrap()
        }

        Screen {
            width: width,
            height: heigth,
            pixel_size: width / 64,
            screen_data: vec![0; (width * heigth) as usize],
            event_pump: create_event_pump(),
            canvas: create_canvas(width, heigth),
            force_close: false,
            started: false,
            screen_buffer: vec![0; (width * heigth) as usize],
        }
    }
}