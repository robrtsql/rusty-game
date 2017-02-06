#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sdl2;

use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::*;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
mod ase;

const SCREEN_FPS: u32 = 60;
const SCREEN_TICKS_PER_FRAME: u32 = 1000 / SCREEN_FPS;
const MAX_DELTA_TIME: f32 = 0.05;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();

    let window = video_subsystem.window("rustgame", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut timer = sdl_context.timer().unwrap();

    let sheet = ase::import("character_idle", &renderer);

    println!("Got {}", sheet.aseprite.meta.image);

    let ref frame = sheet.aseprite.frames["character_idle 0.ase"].frame;
    let mut source_rect = Rect::new(frame.x, frame.y, frame.w, frame.h);
    let zoom = 2;
    let mut dest_rect = Rect::new(frame.x, frame.y, frame.w * zoom, frame.h * zoom);
    dest_rect.center_on(Point::new(400, 300));

    renderer.copy(&sheet.image, Some(source_rect), Some(dest_rect)).expect("Render failed");
    renderer.present();

    let mut keep_playing = true;
    while keep_playing {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    keep_playing = false;
                }
                _ => {}
            }
        }
        let mut dt = 0.016;
        let start_ticks = timer.ticks();

        // do the game logic

        dt = sleep_til_next_frame(&mut timer, start_ticks);
    }
}

fn sleep_til_next_frame(timer: &mut sdl2::TimerSubsystem, start_ticks: u32)  -> f32 {
    let frame_ticks: u32 = timer.ticks() - start_ticks;
    let delay_ticks: u32 = SCREEN_TICKS_PER_FRAME - frame_ticks;

    if delay_ticks > 0 {
        timer.delay(delay_ticks);
    }

    let post_delay_ticks: u32 = timer.ticks() - start_ticks;
    let post_delay_ticks_float = post_delay_ticks as f32;
    let unbounded_delta_time: f32 = post_delay_ticks_float / 1000.0;
    let delta_time: f32 = if unbounded_delta_time > MAX_DELTA_TIME {
        MAX_DELTA_TIME
    } else {
        unbounded_delta_time
    };

    return delta_time;
}