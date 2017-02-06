use serde_json::*;
use ::std::collections::HashMap;
use ::std::fs::File;
use ::std::path::Path;
use sdl2::image::*;
use sdl2::render::*;

// TODO: Reorganize struct for
// sprite sheet/move to new module
// in order to make it easier to work with?
#[derive(Debug)]
pub struct Sheet {
    pub aseprite: Aseprite,
    pub image: Texture,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aseprite {
    pub frames: HashMap<String, Frame>,
    pub meta: Meta,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: Rect,
    pub sourceSize: Size,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Size,
    pub scale: String,
    pub frameTags: Vec<FrameTag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrameTag {
    pub name: String,
    pub from: u32,
    pub to: u32,
    pub direction: String,
}

pub fn import(filename: &str, renderer: &Renderer) -> Sheet {
    let json_file_name = format!("{}.json", filename);
    let file = File::open(format!("assets/{}", json_file_name)).unwrap();
    let aseprite: Aseprite = de::from_reader(file).unwrap();
    let image = renderer.load_texture(Path::new(&aseprite.meta.image)).unwrap();
    let sheet = Sheet { 
        aseprite: aseprite, 
        image: image
        };
    return sheet;
}