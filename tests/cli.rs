#[test]
fn test_find_matches() {
    let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

    let pattern = "safe|productive";
    let mut result = Vec::new();
    grss_clone::find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "safe, fast, productive.\n");
}

#[test]
fn test_find_matches_no_match() {
    let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

    let pattern = "notfound";
    let mut result = Vec::new();
    grss_clone::find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_find_matches_invalid_regex() {
    let content = "Rust";
    let pattern = "("; // Invalid regex
    let mut result = Vec::new();
    let error = grss_clone::find_matches(content, pattern, &mut result).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
