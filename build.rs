use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if entry_path.is_dir() {
            copy_dir_all(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir).join("../../../assets");
    let source_dir = PathBuf::from("assets");

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).unwrap();
    }

    fs::create_dir_all(&target_dir).unwrap();
    copy_dir_all(&source_dir, &target_dir).unwrap();
}
