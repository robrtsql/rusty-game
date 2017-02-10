use std::collections::HashMap;
use std::path::Path;
use std::cell::RefCell;
use sdl2::image::*;
use sdl2::render::*;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use ase::*;

// Represents a set of animations and what is currently playing
pub struct SpriteAnimator<'a> {
    sheet: &'a Sheet,
    playback: RefCell<Playback>,
}

// Represents an .ase file, or a set of animations
pub struct Sheet {
    pub image: Texture,
    pub name: String,
    pub anims: HashMap<String, Vec<Frame>>,
}

// Represents the current playback state of an animator instance
#[derive(Debug, Clone)]
pub struct Playback {
    current_anim: String,
    duration: f32,
    current_frame_index: usize,
}

impl<'a> SpriteAnimator<'a> {
    pub fn render(&self, x: i32, y: i32, zoom: u32, dt: f32, renderer: &mut Renderer) {
        self.update_frame_index(dt);

        {
            let ref playback = self.playback.borrow();
            let ref current_frame = self.sheet.anims[&playback.current_anim]
                .get(playback.current_frame_index as usize)
                .unwrap()
                .frame;
            let source_rect = Rect::new(current_frame.x,
                                        current_frame.y,
                                        current_frame.w,
                                        current_frame.h);
            let mut dest_rect = Rect::new(current_frame.x,
                                          current_frame.y,
                                          current_frame.w * zoom,
                                          current_frame.h * zoom);
            dest_rect.center_on(Point::new(x, y));
            renderer.copy(&self.sheet.image, Some(source_rect), Some(dest_rect))
                .expect("Render failed");
            renderer.present();
            renderer.clear();
        }
    }

    pub fn update_frame_index(&self, dt: f32) {
        let ref mut playback = self.playback.borrow_mut();
        playback.duration += dt * 1000.0;

        let ref anims = self.sheet.anims;
        let ref current_anim = anims.get(&playback.current_anim).unwrap();
        let mut current_frame_duration =
            _get_current_frame_duration(playback.current_frame_index, &current_anim);
        while playback.duration > current_frame_duration {
            playback.duration -= current_frame_duration;
            playback.current_frame_index = (playback.current_frame_index + 1) %
                                           current_anim.len();
            current_frame_duration =
                _get_current_frame_duration(playback.current_frame_index, &current_anim)
        }
    }
}

fn _get_current_frame_duration(index: usize, current_anim: &Vec<Frame>) -> f32 {
    return current_anim.get(index).unwrap().duration as f32;
}

pub fn import_sheet(filename: &str, renderer: &Renderer) -> Sheet {
    let aseprite = import(filename);
    let image = renderer.load_texture(Path::new(&aseprite.meta.image)).unwrap();
    let mut anim_map = HashMap::new();

    for anim in &aseprite.meta.frameTags {
        let mut frames = Vec::new();
        // TODO: handle animations with direction 'backward'
        if anim.direction == "backward" {
            panic!("backwards animations not supported");
        }
        for frame_number in anim.from..(anim.to + 1) {
            frames.push(aseprite.frames[&format!("{} {}.ase", filename, frame_number)].clone());
        }
        anim_map.insert(anim.name.clone(), frames);
    }

    return Sheet {
        image: image,
        name: filename.to_string(),
        anims: anim_map,
    };
}

pub fn get_animator<'a>(sheet: &'a Sheet) -> SpriteAnimator<'a> {
    return SpriteAnimator {
        sheet: sheet,
        playback: RefCell::new(Playback {
            current_anim: "Idle".to_string(),
            duration: 0.0,
            current_frame_index: 0,
        }),
    };
}
