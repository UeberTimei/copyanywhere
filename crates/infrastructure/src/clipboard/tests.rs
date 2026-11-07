use super::{has_text, sequence};

#[test]
fn sequence_is_callable() {
    let _ = sequence();
}

#[test]
fn has_text_does_not_panic() {
    let _ = has_text();
}
