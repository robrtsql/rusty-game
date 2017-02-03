use ::std::process;

pub fn import(filename: &str) {
    ::std::process::Command::new("ls")
            .arg("-l")
            .arg("-a")
            .spawn()
            .expect("ls command failed to start");
}