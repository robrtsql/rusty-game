use serde_json::*;
use ::std::collections::HashMap;
use ::std::fs::File;
use ::std::path::Path;
use sdl2::image::*;
use sdl2::render::*;
use sdl2::rect::Rect;
use sdl2::rect::Point;

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
            let ref current_frame = self.anims["Idle"].get(current_frame_index as usize).unwrap().frame;
            let source_rect = Rect::new(current_frame.x, current_frame.y, current_frame.w, current_frame.h);
            let zoom = 2;
            let mut dest_rect = Rect::new(current_frame.x, current_frame.y, current_frame.w * zoom, current_frame.h * zoom);
            dest_rect.center_on(Point::new(400, 300));
            renderer.copy(&self.image, Some(source_rect), Some(dest_rect)).expect("Render failed");
            renderer.present();
            renderer.clear();
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aseprite {
    pub frames: HashMap<String, Frame>,
    pub meta: Meta,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub frame: Quad,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: Quad,
    pub sourceSize: Size,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quad {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Size,
    pub scale: String,
    pub frameTags: Vec<FrameTag>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrameTag {
    pub name: String,
    pub from: u32,
    pub to: u32,
    pub direction: String,
}

pub fn import<'a>(filename: &str, renderer: &Renderer) -> Sheet {
    let json_file_name = format!("{}.json", filename);
    let file = File::open(format!("assets/{}", json_file_name)).unwrap();
    let aseprite: Aseprite = de::from_reader(file).unwrap();
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