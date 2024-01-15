pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // TODO: we need to handle the heading "/" char otherwise split_once returns an empty string
    // witch is a performance penalty as unnecessary recursive checks are made
    let prefix_str = prefix.split_once('/');
    let request_str = request_path.split_once('/');

    // TODO: i guess the exercise should be done using iterators but it's not clear
    match (prefix_str, request_str) {
        (Some(("*", p_rest)), Some((_, r_rest))) => prefix_matches(p_rest, r_rest),
        (Some((p, p_rest)), Some((r, r_rest))) if p == r => prefix_matches(p_rest, r_rest),
        (None, Some((r, _))) if prefix == r => true,
        (None, _) => prefix == request_path,
        _ => false,
    }
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
