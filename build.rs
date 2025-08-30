use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

// Copy res folder to where cargo creates exe file
fn main() -> Result<()> {
    // This tells Cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["res/"];

    let out_dir = env::var("OUT_DIR")?;
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
