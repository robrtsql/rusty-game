use std::collections::HashMap;
use ase::*;
use graphics::Graphics;

// Represents a set of animations and what is currently playing
#[derive(Copy, Clone, PartialEq)]
pub struct SpriteAnimator<'a> {
    pub sheet: &'a Sheet,
    pub playback: Playback<'a>,
}

// Represents an .ase file, or a set of animations
pub struct Sheet {
    pub name: String,
    pub texture_path: String,
    pub anims: HashMap<String, Vec<Frame>>,
}

impl PartialEq for Sheet {
    fn eq(&self, other: &Sheet) -> bool {
        if self.name == other.name && self.texture_path == other.texture_path {
            for (key, val) in self.anims.iter() {
                match other.anims.get(key) {
                    Some(other_val) => {
                        if (*val).ne(other_val) { return false; }
                    },
                    None => return false,
                }
            }
            return true;
        }
        return false;
    }
}

// Represents the current playback state of an animator instance
#[derive(Copy, Clone, PartialEq)]
pub struct Playback<'a> {
    pub current_anim: &'a str,
    pub duration: f32,
    pub current_frame_index: usize,
}

impl<'a> SpriteAnimator<'a> {
    pub fn update_frame_index(&mut self, dt: f32) {
        let ref mut playback = &mut self.playback;
        playback.duration += dt * 1000.0;

        let ref anims = self.sheet.anims;
        let ref current_anim = anims.get(playback.current_anim).unwrap();
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

pub fn import_sheet(filename: &str, graphics: &mut Graphics) -> Sheet {
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

    return Sheet {
        name: filename.to_string(),
        texture_path: texture_path,
        anims: anim_map,
    };
}

pub fn get_animator<'a>(sheet: &'a Sheet) -> SpriteAnimator<'a> {
    return SpriteAnimator {
        sheet: sheet,
        playback: Playback {
            current_anim: "Idle",
            duration: 0.0,
            current_frame_index: 0,
        },
    };
}
