use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let _ = embed_resource::compile("assets/images/icon.rc", embed_resource::NONE);

    // Path to your assets directory
    let asset_dir = PathBuf::from("assets");
    let target_dir = PathBuf::from(out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("assets");

    // Remove old assets if any, then copy fresh
    let _ = fs::remove_dir_all(&target_dir);
    fs::create_dir_all(&target_dir).unwrap();
    copy_dir(&asset_dir, &target_dir);
}

fn copy_dir(from: &PathBuf, to: &PathBuf) {
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let dest = to.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest).unwrap();
            copy_dir(&path, &dest);
        } else {
            fs::copy(&path, &dest).unwrap();
        }
    }
}
