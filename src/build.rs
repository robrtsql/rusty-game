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

// Rust is great because it prevents us from having unassigned
// references, but it also makes it hard for us to optionally
// do something if and only if many Option<>s are not None.
// For each Option<>, we have to introduce a new scope, and
// suddenly our code has 8 or 9 tabs/indents. Instead, I opted
// to put each new binding of the Some() value into a new
// function. And I named them numerically. Please forgive me.

fn decide_if_needs_rebuilt(file: &AseFile) -> bool {
    match file.ase {
        Some(ref ase_meta) => {
            return decide_if_needs_rebuilt_2(file, ase_meta);
        },
        None => {
            // If there's no .ase file, there's nothing to build
            return false;
        }
    }
}

fn decide_if_needs_rebuilt_2(file: &AseFile, ase_meta: &AseFileMetadata) -> bool {
    match file.png {
        Some(ref png_meta) => {
            return decide_if_needs_rebuilt_3(file, ase_meta, png_meta);
        },
        None => {
            return true;
        }
    }
}

fn decide_if_needs_rebuilt_3(file: &AseFile, ase_meta: &AseFileMetadata, png_meta: &AseFileMetadata) -> bool {
    match file.json {
        Some(ref json_meta) => {
            return decide_if_needs_rebuilt_4(ase_meta, png_meta, json_meta);
        },
        None => {
            return true;
        }
    }
}

fn decide_if_needs_rebuilt_4(ase_meta: &AseFileMetadata, png_meta: &AseFileMetadata, json_meta: &AseFileMetadata) -> bool {
    return ase_meta.modified > png_meta.modified || ase_meta.modified > json_meta.modified;
}

pub fn main() {
    let needs_rebuilt_list = get_ase_needs_rebuilt();
    for os_filename in needs_rebuilt_list {
        let filename = os_filename.to_str().unwrap();
        let json_file_name = format!("assets/{}.json", filename);
        ::std::process::Command::new("aseprite")
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