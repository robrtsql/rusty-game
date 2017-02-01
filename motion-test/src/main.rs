extern crate num_traits;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sdl2;

use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::*;
use sdl2::render::*;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use num_traits::Float;

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

    let mut renderer = window.renderer()
    	.present_vsync()
	.build()
	.unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    let image = renderer.load_texture(Path::new("assets/car.png")).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut timer = sdl_context.timer().unwrap();

    let mut x = 0.0;

    let mut dt = 0.0;
    let mut dt_accumulator = 0.0;
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
        let start_ticks = timer.ticks();

	dt_accumulator = dt_accumulator + dt;
	let fixed_step = 1.0 / 60.0;
	while (dt_accumulator >= fixed_step) {
	     x = physics_update(x, fixed_step);
	     dt_accumulator = dt_accumulator - fixed_step;
	}

	render_update(x, &image, &mut renderer, dt_accumulator);
        dt = sleep_til_next_frame(&mut timer, start_ticks);
    }
}

fn physics_update(x: f32, dt: f32) -> f32 {
    let new_x = x + (150.0 * dt);
    if new_x > 800.0 {
    	return 0.0;
    }
    return new_x;
}

fn render_update(x: f32, image: &Texture, renderer: &mut Renderer, dt: f32) {
    println!("{:32}\n", dt);
    let extrapolated_x = x + (150.0 * dt);
    let mut dest_rect = Rect::new(0,
      			          0,
				  152,
				  62);
    dest_rect.center_on(Point::new(extrapolated_x.round() as i32, 300));
    renderer.copy(&image, None, Some(dest_rect)).expect("Render failed");

    renderer.present();
    renderer.clear();
}

fn sleep_til_next_frame(timer: &mut sdl2::TimerSubsystem, start_ticks: u32) -> f32 {
    let frame_ticks: u32 = timer.ticks() - start_ticks;
    let delay_ticks: i32 = SCREEN_TICKS_PER_FRAME as i32 - frame_ticks as i32;

    // attempt to lock fps to 60
    if delay_ticks > 0 {
        timer.delay(delay_ticks as u32);
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
