use std::collections::HashMap;
use std::path::Path;
use sdl2::image::*;
use sdl2::render::*;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use ase::*;

// TODO: Reorganize struct for
// sprite sheet/move to new module
// in order to make it easier to work with?
pub struct Sheet {
    pub image: Texture,
    pub name: String,
    pub anims: HashMap<String, Vec<Frame>>,
    playback: Playback,
}

pub struct Playback {
    current_anim: String,
    duration: f32,
    current_frame_index: i32,
}

impl Sheet {
    pub fn render(&mut self, renderer: &mut Renderer, dt: f32) {
        self.playback.duration += dt * 1000.0;
        //let current_frame_index = (((self.playback.duration * 1000.0) % 200.0) / 100.0).floor();
        self.update_frame_index();

        {
            let ref current_frame = self.anims[&self.playback.current_anim]
                .get(self.playback.current_frame_index as usize)
                .unwrap()
                .frame;
            let source_rect = Rect::new(current_frame.x,
                                        current_frame.y,
                                        current_frame.w,
                                        current_frame.h);
            let zoom = 2;
            let mut dest_rect = Rect::new(current_frame.x,
                                          current_frame.y,
                                          current_frame.w * zoom,
                                          current_frame.h * zoom);
            dest_rect.center_on(Point::new(400, 300));
            renderer.copy(&self.image, Some(source_rect), Some(dest_rect)).expect("Render failed");
            renderer.present();
            renderer.clear();
        }
    }

    pub fn update_frame_index(&mut self) {
        let ref current_anim = self.anims.get(&self.playback.current_anim).unwrap();
        while self.playback.duration >
              current_anim.get(self.playback.current_frame_index as usize).unwrap().duration as
              f32 {
            self.playback.duration -= current_anim.get(self.playback.current_frame_index as usize).unwrap().duration as f32;
            self.playback.current_frame_index = (self.playback.current_frame_index + 1) % current_anim.len() as i32;
        }
    }
}

pub fn import_anim(filename: &str, renderer: &Renderer) -> Sheet {
    let aseprite = ::ase::import(filename);
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

    println!("{:?}", anim_map);

    let sheet = Sheet {
        image: image,
        name: filename.to_string(),
        anims: anim_map,
        playback: Playback {
            current_anim: "Idle".to_string(),
            duration: 0.0,
            current_frame_index: 0,
        },
    };
    return sheet;
}
