use super::plain_notes;

#[test]
fn headings_and_bullets_lose_their_markdown() {
    let notes = "## What's Changed\n* feat: add updates by @me\n\n**Full Changelog**: x";
    assert_eq!(
        plain_notes(notes),
        "What's Changed\nfeat: add updates by @me\n\n**Full Changelog**: x"
    );
}
