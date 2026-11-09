use serde_json::json;

use super::parse;

#[test]
fn parses_version_notes_and_both_assets() {
    let release = parse(&json!({
        "tag_name": "v1.2.0",
        "body": " Fixed things \n",
        "assets": [
            {"name": "CopyAnywhere.exe", "browser_download_url": "https://x/exe"},
            {"name": "CopyAnywhere.exe.sha256", "browser_download_url": "https://x/sha"},
        ],
    }))
    .unwrap();
    assert_eq!(release.version, "v1.2.0");
    assert_eq!(release.notes, "Fixed things");
    assert_eq!(release.exe_url, "https://x/exe");
    assert_eq!(release.checksum_url, "https://x/sha");
}

#[test]
fn release_without_the_exe_is_rejected() {
    let release = json!({"tag_name": "v1.2.0", "assets": []});
    assert!(parse(&release).is_none());
}

#[test]
fn non_https_asset_is_rejected() {
    let release = json!({
        "tag_name": "v1.2.0",
        "assets": [
            {"name": "CopyAnywhere.exe", "browser_download_url": "http://x/exe"},
            {"name": "CopyAnywhere.exe.sha256", "browser_download_url": "https://x/sha"},
        ],
    });
    assert!(parse(&release).is_none());
}
