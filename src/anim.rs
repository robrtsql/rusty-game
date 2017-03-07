use std::collections::HashMap;
use std::cell::RefCell;
use ase::*;
use graphics::Graphics;

// Represents a set of animations and what is currently playing
#[derive(Clone, Debug)]
pub struct SpriteAnimator {
    pub sheet: Sheet,
    pub playback: RefCell<Playback>,
}

// Represents an .ase file, or a set of animations
#[derive(Clone, Debug)]
pub struct Sheet {
    pub name: String,
    pub texture_path: String,
    pub anims: HashMap<String, Vec<Frame>>,
}

// Represents the current playback state of an animator instance
#[derive(Clone, Debug)]
pub struct Playback {
    pub current_anim: String,
    pub duration: f32,
    pub current_frame_index: usize,
}

impl SpriteAnimator {
    pub fn update_frame_index(&self, dt: f32) {
        let ref mut playback = self.playback.borrow_mut();
        playback.duration += dt * 1000.0;

        let ref anims = self.sheet.anims;
        let ref current_anim = anims.get(&playback.current_anim).unwrap();
        let mut current_frame_duration = _get_current_frame_duration(playback.current_frame_index,
                                                                     &current_anim);
        while playback.duration > current_frame_duration {
            playback.duration -= current_frame_duration;
            playback.current_frame_index = (playback.current_frame_index + 1) % current_anim.len();
            current_frame_duration = _get_current_frame_duration(playback.current_frame_index,
                                                                 &current_anim)
        }
    }
}

fn _get_current_frame_duration(index: usize, current_anim: &Vec<Frame>) -> f32 {
    return current_anim.get(index).unwrap().duration as f32;
}

pub fn import_animator(filename: &str, graphics: &mut Graphics) -> SpriteAnimator {
    let aseprite = import(filename);
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

    let texture_path = format!("assets/{}.png", filename);
    graphics.load_texture(texture_path.clone());

    return SpriteAnimator {
        sheet: Sheet {
            name: filename.to_string(),
            texture_path: texture_path,
            anims: anim_map,
        },
        playback: RefCell::new(Playback {
            current_anim: "Idle".to_string(),
            duration: 0.0,
            current_frame_index: 0,
        }),
    };
}
