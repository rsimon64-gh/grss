use grss_clone::find_matches;

#[test]
fn test_find_matches_integration() {
    let content = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";

    let pattern = "safe|productive";
    let mut result = Vec::new();
    find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "safe, fast, productive.\n");
}
