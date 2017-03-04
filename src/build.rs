extern crate os_type;

use std::fs;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug)]
struct AseFile {
    ase: Option<AseFileMetadata>,
    png: Option<AseFileMetadata>,
    json: Option<AseFileMetadata>,
}

#[derive(Clone, Debug)]
struct AseFileMetadata {
    modified: SystemTime,
}

fn ase_metadata(path: &PathBuf) -> Option<AseFileMetadata> {
    let metadata = fs::metadata(path).unwrap();
    return Some(AseFileMetadata {
        modified: metadata.modified().unwrap(),
    });
}

fn get_ase_needs_rebuilt() -> Vec<OsString> {
    let mut ase_map: HashMap<OsString, AseFile> = HashMap::new();
    for path_result in fs::read_dir("./assets").unwrap() {
        let path = path_result.unwrap().path();
        let stem = path.file_stem().unwrap().to_owned();
        let extension = path.extension().unwrap().to_owned();
        if !ase_map.contains_key(&stem) {
            ase_map.insert(stem.clone(), AseFile {
                ase: None,
                png: None,
                json: None,
            });
        }
        match extension.to_str().unwrap() {
            "ase" => ase_map.get_mut(&stem).unwrap().ase = ase_metadata(&path),
            "png" => ase_map.get_mut(&stem).unwrap().png = ase_metadata(&path),
            "json" => ase_map.get_mut(&stem).unwrap().json = ase_metadata(&path),
            _ => println!("Other file found!!"),
        }
    }

    let mut needs_rebuilt_list = Vec::new();

    for (stem, file) in ase_map.iter() {
        if decide_if_needs_rebuilt(&file) {
            needs_rebuilt_list.push(stem.clone());
        }
    }

    return needs_rebuilt_list;
}

fn decide_if_needs_rebuilt(file: &AseFile) -> bool {
    match *file {
        // If we aren't missing any files, then we only need to build
        // if the .ase file has been updated since the last build
        AseFile { ase: Some(ref ase_meta), png: Some(ref png_meta), json: Some(ref json_meta) } => {
            return ase_meta.modified > png_meta.modified || ase_meta.modified > json_meta.modified;
        },
        AseFile { ase: None, .. } => {
            // If we don't have an .ase file, then there's nothing to build
            return false;
        }
        _ => {
            return true;
        }
    }
}

pub fn main() {
    let needs_rebuilt_list = get_ase_needs_rebuilt();
    for os_filename in needs_rebuilt_list {
        let filename = os_filename.to_str().unwrap();
        let json_file_name = format!("assets/{}.json", filename);
        let aseprite_command = match os_type::current_platform() {
            os_type::OSType::Windows => "C:/Program Files (x86)/Aseprite/Aseprite.exe",
            _ => "aseprite"
        };
        ::std::process::Command::new(aseprite_command)
                .arg(format!("assets/{}.ase", filename))
                .arg("--sheet")
                .arg(format!("assets/{}.png", filename))
                .arg("--data")
                .arg(&json_file_name)
                .arg("--batch")
                .arg("--list-tags")
                .spawn()
                .expect("aseprite export command failed");
    }
}