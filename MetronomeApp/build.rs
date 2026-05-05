use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR should be set by Cargo");

    #[cfg(target_os = "windows")]
    let _ = embed_resource::compile("assets/images/icon.rc", embed_resource::NONE);

    let asset_dir = PathBuf::from("assets");

    if !asset_dir.exists() {
        return;
    }

    let target_dir = PathBuf::from(out_dir)
        .ancestors()
        .nth(3)
        .expect("Cargo OUT_DIR should contain target profile directory")
        .join("assets");

    let _ = fs::remove_dir_all(&target_dir);
    fs::create_dir_all(&target_dir).expect("failed to create target assets directory");

    copy_dir(&asset_dir, &target_dir);
}

fn copy_dir(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).expect("failed to read assets directory") {
        let entry = entry.expect("failed to read asset entry");
        let path = entry.path();
        let dest = to.join(entry.file_name());

        if path.is_dir() {
            fs::create_dir_all(&dest).expect("failed to create nested asset directory");
            copy_dir(&path, &dest);
        } else {
            fs::copy(&path, &dest).expect("failed to copy asset file");
        }
    }
}
