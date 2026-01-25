use std::path::PathBuf;
use eda_ffprobe::ffprobe;

fn get_asset_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("video_assets");
    path.push(filename);
    path
}

#[test]
fn test_ffprobe_parse() {
    let video_av1 = get_asset_path("video_av1_test.mp4");
    let video = ffprobe(&video_av1).unwrap();

    assert_eq!(video.format.format_name, "mov,mp4,m4a,3gp,3g2,mj2");
    assert_eq!(video.format.duration.unwrap(), 3.0);
}