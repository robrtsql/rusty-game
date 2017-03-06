use serde_json::*;
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Aseprite {
    pub frames: HashMap<String, Frame>,
    pub meta: Meta,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Frame {
    pub frame: Quad,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: Quad,
    pub sourceSize: Size,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Quad {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Size,
    pub scale: String,
    pub frameTags: Vec<FrameTag>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FrameTag {
    pub name: String,
    pub from: u32,
    pub to: u32,
    pub direction: String,
}

pub fn import(filename: &str) -> Aseprite {
    let json_file_name = format!("{}.json", filename);
    let file = File::open(format!("assets/{}", json_file_name)).unwrap();
    let aseprite: Aseprite = de::from_reader(file).unwrap();
    return aseprite;
}
