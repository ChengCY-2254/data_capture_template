use std::io::Write;
use std::process::Command;

fn main() {
    let mut build_info = std::fs::File::create("./src/build_info.rs").unwrap();
    let git_command = "git rev-parse HEAD";
    let res = Command::new("sh")
        .arg("-c")
        .arg(git_command)
        .output()
        .unwrap();
    let git_hash = String::from_utf8(res.stdout).unwrap().trim().to_string();
    let git_hash_7 = &git_hash[0..7];
    let build_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let content = format!(
        r#"
    #![allow(dead_code)]
    // This file is auto generated by build.rs 
    pub const GIT_HASH:&str = "{}";
    pub const GIT_HASH_7:&str = "{}";
    pub const BUILD_TIME:&str = "{}";
    "#,
        git_hash, git_hash_7,build_time
    );
    build_info.write_all(content.as_bytes()).unwrap();
}
