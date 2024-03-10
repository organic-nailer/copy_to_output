use std::result::Result::Ok;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::path::Path;
use anyhow::*;

pub fn copy_to_output(path: &str, build_type: &str) -> Result<()> {
    let out_path = get_cargo_target_dir().expect("Could not get cargo target directory");

    // Overwrite existing files with same name
    let mut options = CopyOptions::new();
    options.overwrite = true;

    let mut from_path = Vec::new();
    from_path.push(path);
    copy_items(&from_path, &out_path, &options)?;

    Ok(())
}

pub fn copy_to_output_path(path: &Path, build_type: &str) -> Result<()> {
    let path_str = path.to_str().expect("Could not convert file path to string");
    copy_to_output(path_str, build_type)?;

    Ok(())
}

// Credit to ssrlive for this function
// Taken from the following issue: https://github.com/rust-lang/cargo/issues/9661#issuecomment-1722358176
fn get_cargo_target_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}
