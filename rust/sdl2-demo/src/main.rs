extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("SDL2 DEMO", 1000, 800)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();

  canvas.set_draw_color(Color::RGB(0, 255, 255));
  canvas.clear();
  canvas.present();

  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut i: u8 = 0;
  let mut add = true;
  'running: loop {
    if i >= 255 {
      add = false;
    } else if i <= 0 {
      add = true;
    }
    if add {
      i = i + 5;
    } else {
      i = i - 5
    }
    canvas.set_draw_color(Color::RGB(i, i / 2, 255 - i));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
      .fill_rect(Rect::new(0, 0, 200, 200))
      .expect("Failed to draw rect");
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
    // The rest of the game loop goes here...

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
