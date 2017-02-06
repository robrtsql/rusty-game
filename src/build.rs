pub fn main() {
    let filename: &str = "character_idle";
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