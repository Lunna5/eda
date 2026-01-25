use regex::Regex;


/// Extracts season number, episode number, and optional quality from a TV episode file name.
/// Supported formats include:
/// - S01E01.1080.mp4
/// - S01E01 [1080].mkv
/// - S01E01-1080.webm
/// - S1E1 720.mp4
///
/// # Arguments
/// * `file_name` - The name of the file to parse.
///
/// # Returns
/// An `Option` containing a tuple with season number, episode number, and optional tag string
/// if parsing is successful; otherwise, `None`.
fn parse_episode_info(file_name: &str) -> Option<(u32, u32, Option<String>)> {
    let re = Regex::new(
        r"(?i)\bS(?P<s>\d{1,2})E(?P<e>\d{1,3})\b(?:[.\-\s_\]]*\[?(?P<q>\d{3,4}p?)]?)?"
    ).unwrap();

    re.captures(file_name).map(|caps| {
        let s = caps.name("s").unwrap().as_str().parse::<u32>().ok()?;
        let e = caps.name("e").unwrap().as_str().parse::<u32>().ok()?;
        let q = caps.name("q").map(|m| m.as_str().to_string());
        Some((s, e, q))
    }).flatten()
}

#[cfg(test)]
mod tests {
    use super::parse_episode_info;

    #[test]
    fn test_parse_episode_info() {
        let cases = vec![
            ("S01E01.1080.mp4", Some((1, 1, Some("1080".to_string())))),
            ("S02E10 [720p].mkv", Some((2, 10, Some("720p".to_string())))),
            ("S3E5-480.webm", Some((3, 5, Some("480".to_string())))),
            ("S04E12.mkv", Some((4, 12, None))),
            ("s05e03 1080p.avi", Some((5, 3, Some("1080p".to_string())))),
            ("S1E02 Random Title", Some((1, 2, None))),
            ("randomfile.mp4", None),
            ("S1E1.mp4", Some((1, 1, None))),
        ];

        for (input, expected) in cases {
            let info = parse_episode_info(input);
            assert_eq!(info, expected);
        }
    }
}