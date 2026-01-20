use std::path::PathBuf;

fn get_asset_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("assets");
    path.push(filename);
    path
}
