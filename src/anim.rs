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
    pub aseprite: Aseprite,
    pub image: Texture,
    pub name: String,
    pub anims: HashMap<String, Vec<Frame>>,
    duration: f32,
}

impl Sheet {
    pub fn render(&mut self, renderer: &mut Renderer, dt: f32) {
        self.duration += dt;
        let current_frame_index = (((self.duration * 1000.0) % 200.0) / 100.0).floor();

        {
            let ref current_frame =
                self.anims["Idle"].get(current_frame_index as usize).unwrap().frame;
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
        aseprite: aseprite,
        image: image,
        name: filename.to_string(),
        anims: anim_map,
        duration: 0.0,
    };
    return sheet;
}
