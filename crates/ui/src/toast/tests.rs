use super::geometry::{dpi_scale, popup_origin};

#[test]
fn origin_is_offset_from_cursor_at_scale_one() {
    assert_eq!(popup_origin((100, 200), 1.0), (116.0, 216.0));
}

#[test]
fn origin_converts_physical_point_to_logical() {
    assert_eq!(popup_origin((300, 600), 1.5), (216.0, 416.0));
}

#[test]
fn dpi_maps_to_scale() {
    assert_eq!(dpi_scale(96), 1.0);
    assert_eq!(dpi_scale(144), 1.5);
}
