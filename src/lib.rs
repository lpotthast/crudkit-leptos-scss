use include_dir::{include_dir, Dir};
use std::path::PathBuf;

static SCSS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scss");

/// Path must point to a folder which can be deleted and recreated freely!
pub fn generate(path: PathBuf) {
    if path.exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }
    std::fs::create_dir_all(&path).unwrap();

    SCSS_DIR.extract(&path).unwrap();
}
