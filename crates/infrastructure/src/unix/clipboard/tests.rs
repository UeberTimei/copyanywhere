use super::{digest, has_text, sequence};

#[test]
fn equal_text_has_equal_digest() {
    assert_eq!(digest("copy me"), digest("copy me"));
}

#[test]
fn different_text_has_different_digest() {
    assert_ne!(digest("copy me"), digest("copy you"));
}

#[test]
fn clipboard_queries_do_not_panic_without_a_display() {
    let _ = (sequence(), has_text());
}
